// Solution 4: Typestate Query Builder
use std::marker::PhantomData;

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
    fn new() -> QueryBuilder<NoTable> {
        QueryBuilder {
            table: String::new(),
            columns: vec![],
            where_clause: None,
            _state: PhantomData,
        }
    }

    fn from(self, table: &str) -> QueryBuilder<NoSelect> {
        QueryBuilder {
            table: table.to_string(),
            columns: self.columns,
            where_clause: self.where_clause,
            _state: PhantomData,
        }
    }
}

impl QueryBuilder<NoSelect> {
    fn select(self, cols: &[&str]) -> QueryBuilder<Complete> {
        QueryBuilder {
            table: self.table,
            columns: cols.iter().map(|s| s.to_string()).collect(),
            where_clause: self.where_clause,
            _state: PhantomData,
        }
    }
}

impl QueryBuilder<Complete> {
    fn where_clause(mut self, condition: &str) -> QueryBuilder<Complete> {
        self.where_clause = Some(condition.to_string());
        self
    }

    fn to_sql(&self) -> String {
        let base = format!("SELECT {} FROM {}", self.columns.join(", "), self.table);
        match &self.where_clause {
            Some(w) => format!("{} WHERE {}", base, w),
            None => base,
        }
    }
}

fn main() {
    let sql = QueryBuilder::new()
        .from("orders")
        .select(&["id", "amount", "customer_name"])
        .where_clause("amount > 100")
        .to_sql();
    println!("SQL: {}", sql);

    let sql2 = QueryBuilder::new()
        .from("customers")
        .select(&["id", "name"])
        .to_sql();
    println!("SQL: {}", sql2);

    // Won't compile:
    // QueryBuilder::new().to_sql();                         // no method on NoTable
    // QueryBuilder::new().select(&["id"]);                  // no method on NoTable
    // QueryBuilder::new().from("x").to_sql();               // no method on NoSelect
    // QueryBuilder::new().from("x").where_clause("a > 1");  // no method on NoSelect
}
