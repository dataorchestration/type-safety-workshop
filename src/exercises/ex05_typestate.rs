// =============================================================================
// Exercise 4: TYPESTATE — Invalid Queries Can't Be Built
// =============================================================================
// Time: ~15 minutes
//
// STORY: Production incident. A reporting job ran:
//   "SELECT FROM orders WHERE" — no columns, incomplete WHERE.
// The Java builder's .build() did a runtime null check... which wasn't called.
// 500 error on the CEO's dashboard. Monday morning.
//
// Your job: make incomplete queries a COMPILE ERROR.
// The query must have FROM → SELECT before execute() is even visible.
// =============================================================================

use std::marker::PhantomData;

// State markers
struct NoTable;
struct NoSelect;
struct Complete;

// TODO 1: Make QueryBuilder generic over State
// Add PhantomData<State>
struct QueryBuilder {
    table: String,
    columns: Vec<String>,
    where_clause: Option<String>,
}

// TODO 2: Implement QueryBuilder<NoTable>::new()
// Returns QueryBuilder<NoTable> with empty fields
//
// impl QueryBuilder<NoTable> {
//     fn new() -> QueryBuilder<NoTable> { ... }
// }

// TODO 3: Implement .from() on NoTable → returns NoSelect
//     fn from(self, table: &str) -> QueryBuilder<NoSelect> { ... }

// TODO 4: Implement .select() on NoSelect → returns Complete
//     fn select(self, cols: &[&str]) -> QueryBuilder<Complete> { ... }

// TODO 5: Implement these ONLY on Complete:
//     fn where_clause(self, condition: &str) -> QueryBuilder<Complete> { ... }
//     fn to_sql(&self) -> String { ... }
//
// to_sql() should return something like:
//   "SELECT id, name FROM users WHERE age > 18"
// or if no where clause:
//   "SELECT id, name FROM users"

fn main() {
    println!("=== Exercise 4: Typestate Query Builder ===\n");

    // TODO 6: Make this work:
    // let sql = QueryBuilder::new()
    //     .from("orders")
    //     .select(&["id", "amount", "customer_name"])
    //     .where_clause("amount > 100")
    //     .to_sql();
    // println!("SQL: {}", sql);

    // Without where clause:
    // let sql2 = QueryBuilder::new()
    //     .from("customers")
    //     .select(&["id", "name"])
    //     .to_sql();
    // println!("SQL: {}", sql2);

    // TODO 7: Uncomment each. Predict the error. Compile.
    // QueryBuilder::new().to_sql();                         // No table, no select!
    // QueryBuilder::new().select(&["id"]);                  // No table!
    // QueryBuilder::new().from("x").to_sql();               // No select!
    // QueryBuilder::new().from("x").where_clause("a > 1");  // No select!

    println!("  Complete TODOs above.");
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_full_query() {
    //     let sql = QueryBuilder::new()
    //         .from("users")
    //         .select(&["id", "name"])
    //         .where_clause("age > 18")
    //         .to_sql();
    //     assert!(sql.contains("SELECT"));
    //     assert!(sql.contains("users"));
    //     assert!(sql.contains("age > 18"));
    // }

    // #[test]
    // fn test_no_where() {
    //     let sql = QueryBuilder::new()
    //         .from("products")
    //         .select(&["name", "price"])
    //         .to_sql();
    //     assert!(sql.contains("SELECT"));
    //     assert!(!sql.contains("WHERE"));
    // }
}
