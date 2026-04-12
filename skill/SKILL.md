# Skill: Type-Safe Rust Data Pipelines

Build type-safe data pipelines in Rust using four compiler-enforced patterns. Use this skill when writing data engineering code in Rust.

## Patterns

### 1. Newtype — Distinct domain types from primitives
Use for: IDs, units, validated strings — anything where two values have the same underlying type but different domain meaning.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CustomerId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct OrderId(u64);

// Validated construction — reject bad data at the boundary
#[derive(Debug, Clone)]
struct ColumnName(String);

impl ColumnName {
    fn new(name: &str) -> Result<Self, String> {
        if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(format!("Invalid column name: '{}'", name));
        }
        Ok(ColumnName(name.to_string()))
    }
    fn as_str(&self) -> &str { &self.0 }
}
```

**Rule:** If two values have the same primitive type but different domain meaning, wrap them. Zero runtime cost.

### 2. Phantom Types — Compile-time stage tracking
Use for: Data lifecycle (Raw → Validated → Clean), connection states, any multi-stage process.

```rust
use std::marker::PhantomData;

struct Raw;
struct Validated;
struct Ready;

struct DataFrame<Stage> {
    data: Vec<Vec<String>>,
    _stage: PhantomData<Stage>,
}

impl DataFrame<Raw> {
    fn validate(self) -> DataFrame<Validated> {
        // self is CONSUMED — old Raw reference is dead
        DataFrame { data: self.data, _stage: PhantomData }
    }
}

impl DataFrame<Validated> {
    fn clean(self) -> DataFrame<Ready> {
        DataFrame { data: self.data, _stage: PhantomData }
    }
}

impl DataFrame<Ready> {
    fn query(&self, sql: &str) -> Vec<String> {
        // Only Ready data can be queried
        vec![]
    }
}
```

**Rule:** If operations should only be valid at certain stages, use PhantomData. Methods only exist on the correct stage. Move semantics prevent using stale data.

### 3. Trait Bounds — Restrict operations to valid types
Use for: Numeric-only operations, type-safe column transforms, composable pipelines.

```rust
// Marker trait
trait Numeric {}
impl Numeric for IntCol {}
impl Numeric for FloatCol {}
// StrCol intentionally does NOT implement Numeric

struct Column<T> {
    name: String,
    data: Vec<String>,
    _type: PhantomData<T>,
}

// sum() only exists on Numeric columns
impl<T: Numeric> Column<T> {
    fn sum(&self) -> f64 {
        self.data.iter().filter_map(|s| s.parse::<f64>().ok()).sum()
    }
}

// uppercase() only exists on StrCol
impl Column<StrCol> {
    fn uppercase(&self) -> Column<StrCol> {
        Column {
            name: self.name.clone(),
            data: self.data.iter().map(|s| s.to_uppercase()).collect(),
            _type: PhantomData,
        }
    }
}

// Type-safe chaining
trait Transform {
    type Input;
    type Output;
    fn apply(&self, input: &Self::Input) -> Self::Output;
}

struct Chain<T1, T2> { first: T1, second: T2 }

impl<T1: Transform, T2: Transform<Input = T1::Output>> Transform for Chain<T1, T2> {
    type Input = T1::Input;
    type Output = T2::Output;
    fn apply(&self, input: &Self::Input) -> Self::Output {
        self.second.apply(&self.first.apply(input))
    }
}
```

**Rule:** If an operation only makes sense on certain types, use a marker trait as a bound. The compiler rejects invalid combinations.

### 4. Typestate — Builder pattern with compile-time step enforcement
Use for: Query builders, connection setup, any multi-step construction where order matters.

```rust
struct NoTable;
struct NoSelect;
struct Complete;

struct QueryBuilder<State> {
    table: String,
    columns: Vec<String>,
    where_clause: Option<String>,
    _state: PhantomData<State>,
}

impl QueryBuilder<NoTable> {
    fn new() -> Self { /* ... */ }
    fn from(self, table: &str) -> QueryBuilder<NoSelect> { /* ... */ }
}

impl QueryBuilder<NoSelect> {
    fn select(self, cols: &[&str]) -> QueryBuilder<Complete> { /* ... */ }
}

impl QueryBuilder<Complete> {
    fn where_clause(mut self, w: &str) -> Self { self.where_clause = Some(w.into()); self }
    fn to_sql(&self) -> String { /* ... */ }
}
```

**Rule:** If a builder has required steps, make each step return a different type. `execute()` / `build()` / `to_sql()` only exists on the terminal state.

## When to apply

| Situation | Pattern |
|-----------|---------|
| Two values with same primitive type, different meaning | Newtype |
| Data must go through stages before use | Phantom Types |
| Operation only valid on certain column/data types | Trait Bounds |
| Multi-step construction with required ordering | Typestate |
| All of the above in one pipeline | Combine all four |

## Key compiler errors to expect

| Error | Meaning | Fix |
|-------|---------|-----|
| `expected CustomerId, found OrderId` | Newtype mismatch | Pass the correct wrapped type |
| `no method named query found for DataFrame<Raw>` | Wrong stage | Progress through pipeline stages |
| `the trait bound StrCol: Numeric is not satisfied` | Invalid operation for type | Only use numeric ops on numeric columns |
| `no method named to_sql found for QueryBuilder<NoTable>` | Skipped builder step | Complete all required steps first |
| `use of moved value` | Old stage consumed | Use the new returned value |

## Design principle

> Parse, don't validate. Once data enters a typed representation, the types carry the proof of validity forever. No re-checking needed.
