// Solution: Option
fn find_customer(id: u64) -> Option<String> {
    match id { 1 => Some("Alice".into()), 2 => Some("Bob".into()), _ => None }
}

fn first_char(s: &str) -> Option<char> { s.chars().next() }

fn main() {
    match find_customer(1) {
        Some(n) => println!("Found: {}", n),
        None => println!("Not found"),
    }
    if let Some(n) = find_customer(2) { println!("Got: {}", n); }
    println!("{}", find_customer(999).unwrap_or("Unknown".into()));
    println!("{:?}", first_char("hello"));  // Some('h')
    println!("{:?}", first_char(""));        // None
    println!("{:?}", find_customer(1).map(|n| n.to_uppercase()));
}
