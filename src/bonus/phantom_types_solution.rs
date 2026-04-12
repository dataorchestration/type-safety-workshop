// Solution 2: Phantom Types
use std::marker::PhantomData;

struct Raw;
struct Validated;
struct Ready;

struct DataBatch<Stage> {
    name: String,
    rows: usize,
    _stage: PhantomData<Stage>,
}

impl DataBatch<Raw> {
    fn load(name: &str, rows: usize) -> DataBatch<Raw> {
        println!("[Load] '{}' — {} rows", name, rows);
        DataBatch { name: name.to_string(), rows, _stage: PhantomData }
    }

    fn validate(self) -> DataBatch<Validated> {
        println!("[Validate] '{}' — checking schema", self.name);
        DataBatch { name: self.name, rows: self.rows, _stage: PhantomData }
    }
}

impl DataBatch<Validated> {
    fn clean(self) -> DataBatch<Ready> {
        println!("[Clean] '{}' — removing nulls", self.name);
        DataBatch { name: self.name, rows: self.rows, _stage: PhantomData }
    }
}

impl DataBatch<Ready> {
    fn query(&self, sql: &str) -> String {
        println!("[Query] '{}' — {}", self.name, sql);
        format!("{} rows from '{}'", self.rows, self.name)
    }
}

fn main() {
    let raw = DataBatch::load("orders.csv", 50_000);
    let validated = raw.validate();
    let ready = validated.clean();
    let result = ready.query("SELECT * WHERE amount > 100");
    println!("Result: {}", result);

    // Won't compile:
    // raw.query("SELECT *");       // no method `query` on DataBatch<Raw>
    // validated.query("SELECT *");  // no method `query` on DataBatch<Validated>
    // ready.validate();             // no method `validate` on DataBatch<Ready>

    println!("\nPhantomData size: {} bytes", std::mem::size_of::<PhantomData<Raw>>());
}
