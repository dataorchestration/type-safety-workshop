// =============================================================================
// Exercise 1: OPTION — Killing the NullPointerException Forever
// =============================================================================
// STORY: 2 AM page. "Users service down." Stack trace shows NullPointerException
// on line 247. Some field was null. Java didn't warn you. Python wouldn't either.
//
// Your job: make "forgot to handle null" a COMPILE ERROR.
// =============================================================================
//
// Option<T> is an enum with two variants:
//   Some(T)  — a value exists
//   None     — no value (the Rust equivalent of null)
//
// The compiler FORCES you to handle both cases. You cannot "just use" the
// value without unwrapping it first.
// =============================================================================

/// Look up a customer by ID. Returns Some(name) if found, None if not.
fn find_customer(id: u64) -> Option<String> {
    match id {
        1 => Some("Alice".to_string()),
        2 => Some("Bob".to_string()),
        _ => None,
    }
}

fn main() {
    println!("=== Exercise 1: Option ===\n");

    let result = find_customer(1);
    // TODO 1: Use `match` to print the name if Some, or "Not found" if None
    //
    // match result {
    //     Some(name) => println!("Found: {}", name),
    //     None => println!("Not found"),
    // }

    // TODO 2: Try find_customer(999) — does it compile if you forget to handle None?
    // Hint: Try `let name: String = find_customer(999);` — compile error!

    // TODO 3: `if let` — cleaner when you only care about the Some case
    // if let Some(name) = find_customer(2) {
    //     println!("Got: {}", name);
    // }

    // TODO 4: .unwrap_or() — provide a default
    // let name = find_customer(999).unwrap_or("Unknown".to_string());
    // println!("{}", name);

    // TODO 5: Write a function `first_char(s: &str) -> Option<char>` that returns
    //         the first character, or None if the string is empty.
    //         Test it with "hello" and "".

    // TODO 6: Chain with .map() — transform the Some value, None stays None
    // let upper = find_customer(1).map(|name| name.to_uppercase());
    // println!("{:?}", upper);
}

// KEY INSIGHT: In Java/Python, a function returning String might return null.
// The caller has no way to know without reading docs (or getting paged at 2 AM).
// In Rust, Option<String> IS the documentation — and the compiler enforces it.
