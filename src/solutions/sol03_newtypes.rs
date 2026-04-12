// Solution 1: Newtypes
#[derive(Debug, Clone, Copy, PartialEq)]
struct CustomerId(u64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct OrderId(u64);

fn lookup_customer(id: CustomerId) -> String {
    format!("Customer #{}", id.0)
}

fn cancel_order(id: OrderId) -> String {
    format!("Cancelled order #{}", id.0)
}

#[derive(Debug, Clone)]
struct ColumnName(String);

impl ColumnName {
    fn new(name: &str) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Column name cannot be empty".to_string());
        }
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(format!("Invalid column name: '{}'", name));
        }
        Ok(ColumnName(name.to_string()))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

fn main() {
    let customer = CustomerId(42);
    let order = OrderId(99);

    println!("{}", lookup_customer(customer));
    println!("{}", cancel_order(order));

    // These would NOT compile:
    // lookup_customer(order);     // expected CustomerId, found OrderId
    // cancel_order(customer);     // expected OrderId, found CustomerId
    // lookup_customer(42_u64);    // expected CustomerId, found u64

    let good = ColumnName::new("customer_id");
    let bad = ColumnName::new("has spaces");
    let empty = ColumnName::new("");
    println!("Good: {:?}", good);
    println!("Bad: {:?}", bad);
    println!("Empty: {:?}", empty);
}
