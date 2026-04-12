// =============================================================================
// Exercise 1: NEWTYPES — Make Wrong Code Impossible
// =============================================================================
// Time: ~15 minutes
//
// STORY: You're building a data pipeline. customer_id and order_id are both u64.
// A junior dev swapped them in a JOIN. It ran for 3 weeks before anyone noticed.
// 10,000 orders linked to wrong customers.
//
// Your job: make that bug a COMPILE ERROR.
// =============================================================================

// TODO 1: Define two newtypes — CustomerId and OrderId
// Both wrap u64. Derive Debug, Clone, Copy, PartialEq.
//
// Example:
//   #[derive(Debug, Clone, Copy, PartialEq)]
//   struct CustomerId(u64);
//
// Your code here:



// TODO 2: Change this function to accept CustomerId instead of u64
fn lookup_customer(id: u64) -> String {
    format!("Customer #{}", id)
}

// TODO 3: Change this function to accept OrderId instead of u64
fn cancel_order(id: u64) -> String {
    format!("Cancelled order #{}", id)
}

// TODO 4: Create a ColumnName newtype with validation
// - Wrap a String
// - Add fn new(name: &str) -> Result<Self, String>
//   - Reject empty strings
//   - Reject strings with spaces or special chars (only alphanumeric + underscore)
// - Add fn as_str(&self) -> &str
//
// Your code here:



fn main() {
    println!("=== Exercise 1: Newtypes ===\n");

    // TODO 5: Fix these to use your newtypes
    let customer = 42_u64;       // Change to: CustomerId(42)
    let order = 99_u64;          // Change to: OrderId(99)

    println!("{}", lookup_customer(customer));
    println!("{}", cancel_order(order));

    // TODO 6: Uncomment after fixing above. Verify this DOESN'T compile:
    // println!("{}", lookup_customer(order));  // WRONG TYPE — should fail!
    // println!("{}", cancel_order(customer));  // WRONG TYPE — should fail!

    // TODO 7: Test your ColumnName
    // let good = ColumnName::new("customer_id");
    // let bad = ColumnName::new("has spaces");
    // let empty = ColumnName::new("");
    // println!("Good: {:?}", good);   // Should be Ok(...)
    // println!("Bad: {:?}", bad);     // Should be Err(...)
    // println!("Empty: {:?}", empty); // Should be Err(...)
}

// =============================================================================
// TESTS — uncomment these after completing the exercise
// Run: cargo test --bin exercise_01_newtypes
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_customer_id() {
    //     let id = CustomerId(42);
    //     assert_eq!(format!("{:?}", id), "CustomerId(42)");
    // }

    // #[test]
    // fn test_order_id() {
    //     let id = OrderId(99);
    //     assert_eq!(format!("{:?}", id), "OrderId(99)");
    // }

    // #[test]
    // fn test_lookup_customer() {
    //     let result = lookup_customer(CustomerId(1));
    //     assert!(result.contains("1"));
    // }

    // #[test]
    // fn test_column_name_valid() {
    //     assert!(ColumnName::new("customer_id").is_ok());
    //     assert!(ColumnName::new("amount_usd").is_ok());
    // }

    // #[test]
    // fn test_column_name_invalid() {
    //     assert!(ColumnName::new("").is_err());
    //     assert!(ColumnName::new("has spaces").is_err());
    //     assert!(ColumnName::new("special!char").is_err());
    // }
}
