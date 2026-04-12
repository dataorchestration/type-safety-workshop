// =============================================================================
// FULL PIPELINE: All Four Patterns Combined
// Run: cargo run --bin full_pipeline
// =============================================================================
// This is what you demo at the END of the workshop to tie everything together.
// =============================================================================

use std::marker::PhantomData;

// ═══════ NEWTYPES (Exercise 1) ═══════
#[derive(Debug, Clone, PartialEq)]
struct ColumnName(String);

impl ColumnName {
    fn new(name: &str) -> Result<Self, String> {
        if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(format!("Invalid: '{}'", name));
        }
        Ok(ColumnName(name.to_string()))
    }
    fn as_str(&self) -> &str { &self.0 }
}

// ═══════ PHANTOM STAGES (Exercise 2) ═══════
struct Raw;
struct Validated;
struct Ready;

struct Dataset<Stage> {
    name: String,
    columns: Vec<ColumnName>,
    rows: Vec<Vec<String>>,
    _stage: PhantomData<Stage>,
}

impl Dataset<Raw> {
    fn load(name: &str, cols: &[&str], rows: Vec<Vec<&str>>) -> Self {
        let columns: Vec<ColumnName> = cols.iter()
            .map(|c| ColumnName::new(c).expect("Bad column name"))
            .collect();
        let rows = rows.into_iter()
            .map(|r| r.into_iter().map(String::from).collect())
            .collect();
        println!("  [Load] '{}': {} cols, {} rows", name, columns.len(), rows.len());
        Dataset { name: name.into(), columns, rows, _stage: PhantomData }
    }

    fn validate(self) -> Dataset<Validated> {
        for (i, row) in self.rows.iter().enumerate() {
            assert_eq!(row.len(), self.columns.len(),
                "Row {} has {} values, expected {}", i, row.len(), self.columns.len());
        }
        println!("  [Validate] '{}' ✓", self.name);
        Dataset { name: self.name, columns: self.columns, rows: self.rows, _stage: PhantomData }
    }
}

impl Dataset<Validated> {
    fn clean(self) -> Dataset<Ready> {
        let rows: Vec<Vec<String>> = self.rows.into_iter()
            .map(|row| row.into_iter().map(|v| v.trim().to_string()).collect())
            .collect();
        println!("  [Clean] '{}' — trimmed whitespace", self.name);
        Dataset { name: self.name, columns: self.columns, rows, _stage: PhantomData }
    }
}

// ═══════ TYPED COLUMNS (Exercise 3) ═══════
struct IntCol;
struct StrCol;

trait Numeric {}
impl Numeric for IntCol {}

struct Column<T> {
    name: ColumnName,
    data: Vec<String>,
    _type: PhantomData<T>,
}

impl<T> Column<T> {
    fn new(name: ColumnName, data: Vec<String>) -> Self {
        Column { name, data, _type: PhantomData }
    }
}

impl<T: Numeric> Column<T> {
    fn sum(&self) -> f64 {
        self.data.iter().filter_map(|s| s.parse::<f64>().ok()).sum()
    }
}

impl Column<StrCol> {
    fn uppercase(&self) -> Column<StrCol> {
        Column::new(self.name.clone(), self.data.iter().map(|s| s.to_uppercase()).collect())
    }
}

impl Dataset<Ready> {
    fn column_int(&self, name: &str) -> Column<IntCol> {
        let idx = self.columns.iter().position(|c| c.as_str() == name)
            .expect(&format!("Column '{}' not found", name));
        let data = self.rows.iter().map(|r| r[idx].clone()).collect();
        Column::new(ColumnName::new(name).unwrap(), data)
    }

    fn column_str(&self, name: &str) -> Column<StrCol> {
        let idx = self.columns.iter().position(|c| c.as_str() == name)
            .expect(&format!("Column '{}' not found", name));
        let data = self.rows.iter().map(|r| r[idx].clone()).collect();
        Column::new(ColumnName::new(name).unwrap(), data)
    }
}

// ═══════ TYPESTATE QUERY (Exercise 4) ═══════
struct QEmpty;
struct QFrom;
struct QComplete;

struct Query<S> {
    table: String,
    cols: Vec<String>,
    filter: Option<String>,
    _s: PhantomData<S>,
}

impl Query<QEmpty> {
    fn new() -> Self { Query { table: String::new(), cols: vec![], filter: None, _s: PhantomData } }
    fn from(self, t: &str) -> Query<QFrom> {
        Query { table: t.into(), cols: self.cols, filter: self.filter, _s: PhantomData }
    }
}

impl Query<QFrom> {
    fn select(self, c: &[&str]) -> Query<QComplete> {
        Query { table: self.table, cols: c.iter().map(|s| s.to_string()).collect(), filter: self.filter, _s: PhantomData }
    }
}

impl Query<QComplete> {
    fn where_clause(mut self, w: &str) -> Self { self.filter = Some(w.into()); self }
    fn to_sql(&self) -> String {
        let base = format!("SELECT {} FROM {}", self.cols.join(", "), self.table);
        match &self.filter {
            Some(f) => format!("{} WHERE {}", base, f),
            None => base,
        }
    }
}

// ═══════ MAIN: THE FULL PIPELINE ═══════

fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║   Type-Safe Analytics Pipeline — All 4 Patterns  ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    // 1. LOAD (Raw)
    println!("── Step 1: Load ──");
    let raw = Dataset::<Raw>::load("orders",
        &["id", "customer", "amount"],
        vec![
            vec!["1", " Alice ", "250"],
            vec!["2", " Bob ",   "100"],
            vec!["3", "Charlie", "500"],
        ],
    );

    // 2. VALIDATE (Raw → Validated)
    println!("\n── Step 2: Validate ──");
    let validated = raw.validate();

    // 3. CLEAN (Validated → Ready)
    println!("\n── Step 3: Clean ──");
    let ready = validated.clean();

    // 4. TYPE-SAFE COLUMN OPERATIONS
    println!("\n── Step 4: Typed Column Operations ──");
    let amounts = ready.column_int("amount");
    let names = ready.column_str("customer");
    println!("  Total amount: {}", amounts.sum());
    println!("  Uppercase: {:?}", names.uppercase().data);
    // amounts.uppercase();  // WON'T COMPILE: IntCol has no uppercase
    // names.sum();          // WON'T COMPILE: StrCol is not Numeric

    // 5. TYPE-SAFE QUERY
    println!("\n── Step 5: Type-Safe Query ──");
    let sql = Query::new()
        .from("orders")
        .select(&["id", "customer", "amount"])
        .where_clause("amount > 100")
        .to_sql();
    println!("  {}\n", sql);
    // Query::new().to_sql();  // WON'T COMPILE: no method on QEmpty

    // SUMMARY
    println!("══════════════════════════════════════════════════");
    println!("  ✓ Column names validated via newtypes");
    println!("  ✓ Pipeline stages enforced via phantom types");
    println!("  ✓ Column ops restricted via trait bounds");
    println!("  ✓ Query construction enforced via typestate");
    println!("  ✓ Zero runtime cost — all checks at compile time");
    println!("  ✓ If it compiles, it's structurally correct.");
    println!("══════════════════════════════════════════════════");
}
