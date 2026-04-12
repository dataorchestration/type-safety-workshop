// =============================================================================
// Exercise 3: TRAIT TRANSFORMS — Incompatible Pipelines Can't Be Built
// =============================================================================
// Time: ~20 minutes
//
// STORY: Your analytics dashboard showed revenue of "AliceBobCharlie".
// Someone called pandas .sum() on the customer_name column instead of amount.
// pandas happily concatenated all strings. No error. Took a week to catch.
//
// Your job: make sum() on a string column a COMPILE ERROR.
// =============================================================================

use std::marker::PhantomData;

// Column type markers
struct IntCol;
struct StrCol;
struct FloatCol;

// TODO 1: Make Column generic — Column<T>
// Add PhantomData<T> to track the column type
struct Column {
    name: String,
    data: Vec<String>,
}

// TODO 2: Implement Column::new as a generic constructor
// fn new(name: &str, data: Vec<&str>) -> Column<T>
impl Column {
    fn new(name: &str, data: Vec<&str>) -> Self {
        Column {
            name: name.to_string(),
            data: data.iter().map(|s| s.to_string()).collect(),
        }
    }
}

// TODO 3: Create a marker trait `Numeric` and implement it ONLY for IntCol and FloatCol
// trait Numeric {}
// impl Numeric for IntCol {}
// impl Numeric for FloatCol {}
// StrCol does NOT get Numeric!

// TODO 4: Implement sum() ONLY for numeric columns
// This should work:  Column<IntCol>::new("x", ...).sum()
// This should fail:  Column<StrCol>::new("x", ...).sum()
//
// impl<T: Numeric> Column<T> {    // <-- note the bound!
//     fn sum(&self) -> f64 {
//         self.data.iter().filter_map(|s| s.parse::<f64>().ok()).sum()
//     }
// }

// TODO 5: Implement uppercase() ONLY for StrCol
// impl Column<StrCol> {
//     fn uppercase(&self) -> Column<StrCol> { ... }
// }

// TODO 6: Implement cast_to_float() ONLY for IntCol
// Input: Column<IntCol>  →  Output: Column<FloatCol>
// This is a type-changing transform!

fn main() {
    println!("=== Exercise 3: Trait Transforms ===\n");

    // TODO 7: Make these work with your generic Column<T>
    // let amount = Column::<IntCol>::new("amount", vec!["100", "250", "50"]);
    // let name = Column::<StrCol>::new("customer", vec!["alice", "bob"]);
    // let price = Column::<FloatCol>::new("price", vec!["9.99", "24.50"]);

    // These should WORK:
    // println!("Amount sum: {}", amount.sum());
    // println!("Price sum: {}", price.sum());
    // let upper = name.uppercase();
    // println!("Uppercase: {:?}", upper.data);
    // let as_float = amount.cast_to_float();

    // These should NOT compile (uncomment to verify):
    // name.sum();              // StrCol is not Numeric!
    // amount.uppercase();      // IntCol has no uppercase!
    // name.cast_to_float();    // Only IntCol can cast to float!

    println!("  Complete TODOs above.");
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_int_sum() {
    //     let col = Column::<IntCol>::new("x", vec!["10", "20", "30"]);
    //     assert_eq!(col.sum(), 60.0);
    // }

    // #[test]
    // fn test_uppercase() {
    //     let col = Column::<StrCol>::new("x", vec!["hello", "world"]);
    //     let upper = col.uppercase();
    //     assert_eq!(upper.data, vec!["HELLO", "WORLD"]);
    // }

    // #[test]
    // fn test_cast() {
    //     let col = Column::<IntCol>::new("x", vec!["42"]);
    //     let _float_col: Column<FloatCol> = col.cast_to_float();
    // }
}
