// =============================================================================
// Exercise 2: RESULT + ENUMS — Errors at the Call Site
// =============================================================================
// STORY: Production job crashes. Stack trace is 50 frames deep. The error
// originated in a function that was called from a function that was called
// from... good luck. In Java, methods throw exceptions you didn't know about.
// In Python, any line can raise.
//
// Your job: make the function signature tell the truth about what can fail,
// and force the caller to handle it.
// =============================================================================
//
// Result<T, E> is an enum with two variants:
//   Ok(T)   — success, carries the value
//   Err(E)  — failure, carries the error
// =============================================================================

// TODO 1: Rust enums can carry data — this is what makes Option and Result work!
// Define a custom error enum:
//
// #[derive(Debug)]
// enum ParseError {
//     Empty,
//     NotANumber(String),
//     TooLarge(i64),
// }

/// Parse a string into an integer, with custom errors
fn parse_positive(s: &str) -> Result<i64, String> {
    if s.is_empty() {
        return Err("empty string".to_string());
    }
    match s.parse::<i64>() {
        Ok(n) if n >= 0 => Ok(n),
        Ok(n) => Err(format!("negative: {}", n)),
        Err(_) => Err(format!("not a number: {}", s)),
    }
}

fn main() {
    println!("=== Exercise 2: Result ===\n");

    // TODO 2: Handle the Result with match
    // match parse_positive("42") {
    //     Ok(n) => println!("Got: {}", n),
    //     Err(e) => println!("Error: {}", e),
    // }

    // TODO 3: Try these cases:
    // parse_positive("42")       // Ok(42)
    // parse_positive("")         // Err("empty string")
    // parse_positive("abc")      // Err("not a number: abc")
    // parse_positive("-5")       // Err("negative: -5")

    // TODO 4: The ? operator — short-circuit on error
    // Write a function that parses two strings and adds them:
    //
    // fn add_two(a: &str, b: &str) -> Result<i64, String> {
    //     let x = parse_positive(a)?;  // if Err, return early
    //     let y = parse_positive(b)?;
    //     Ok(x + y)
    // }
    //
    // Test: add_two("10", "20") → Ok(30)
    //       add_two("10", "abc") → Err(...)

    // TODO 5: Rewrite parse_positive to return your custom ParseError enum
    // instead of String. This is how real Rust code works — typed errors.

    // TODO 6: Try calling parse_positive without handling the Result:
    // let n: i64 = parse_positive("42");  // Compile error!
    //
    // The compiler FORCES you to acknowledge that it might fail.
    // In Java, you can call a throwing method and forget to catch. Not here.
}

// KEY INSIGHT: Result<T, E> is the function signature telling the truth.
// Java's checked exceptions try to do this but nobody uses them.
// Python has no way to express "this function can fail" in the signature.
// Rust makes it mandatory — and ergonomic via the ? operator.
