// =============================================================================
// Exercise 2: PHANTOM TYPES — Data That Knows Its Stage
// =============================================================================
// Time: ~15 minutes
//
// STORY: Your team's data pipeline loads CSV → validates → cleans → queries.
// Last month, someone queried raw data (skipped validation). The report went
// to the CEO with null values showing as "NaN". You got the 2 AM call.
//
// Your job: make querying unvalidated data a COMPILE ERROR.
// =============================================================================

use std::marker::PhantomData;

// Stage markers — zero-size types, exist only for the compiler
struct Raw;
struct Validated;
struct Ready;

// TODO 1: Add PhantomData<Stage> to this struct
// The struct should be generic: DataBatch<Stage>
struct DataBatch {
    name: String,
    rows: usize,
    // Add: _stage: PhantomData<Stage>,
}

// TODO 2: Implement load() — creates a DataBatch<Raw>
// impl DataBatch<Raw> {
//     fn load(name: &str, rows: usize) -> DataBatch<Raw> { ... }
// }
//
// TODO 3: Implement validate() — consumes Raw, returns Validated
//     fn validate(self) -> DataBatch<Validated> { ... }
//
// TODO 4: Implement clean() on Validated — returns Ready
// impl DataBatch<Validated> {
//     fn clean(self) -> DataBatch<Ready> { ... }
// }
//
// TODO 5: Implement query() ONLY on Ready
// impl DataBatch<Ready> {
//     fn query(&self, sql: &str) -> String { ... }
// }

fn main() {
    println!("=== Exercise 2: Phantom Types ===\n");

    // TODO 6: Make this pipeline work:
    // let raw = DataBatch::load("orders.csv", 50_000);
    // let validated = raw.validate();
    // let ready = validated.clean();
    // let result = ready.query("SELECT * WHERE amount > 100");
    // println!("Result: {}", result);

    // TODO 7: Uncomment each line below ONE AT A TIME.
    // Predict the error, then compile to verify.

    // let raw = DataBatch::load("test", 100);
    // raw.query("SELECT *");           // Can you query raw data?

    // let validated = raw.validate();
    // validated.query("SELECT *");     // Can you query before cleaning?

    // let ready = validated.clean();
    // ready.validate();                // Can you go backwards?

    // TODO 8: After validate(), try using the old `raw` variable:
    // let raw = DataBatch::load("test", 100);
    // let validated = raw.validate();
    // println!("{}", raw.rows);        // What happens? Why?

    println!("  Complete TODOs and uncomment the pipeline above.");
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_pipeline_compiles() {
    //     let raw = DataBatch::load("test", 100);
    //     let validated = raw.validate();
    //     let ready = validated.clean();
    //     let result = ready.query("SELECT 1");
    //     assert!(result.contains("test"));
    // }

    // #[test]
    // fn test_zero_size() {
    //     use std::mem::size_of;
    //     assert_eq!(size_of::<PhantomData<Raw>>(), 0);
    //     assert_eq!(size_of::<PhantomData<Validated>>(), 0);
    //     assert_eq!(size_of::<PhantomData<Ready>>(), 0);
    // }
}
