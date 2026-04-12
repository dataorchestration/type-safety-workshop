// Solution: Result + Enums
#[derive(Debug)]
enum ParseError { Empty, NotANumber(String), Negative(i64) }

fn parse_positive(s: &str) -> Result<i64, ParseError> {
    if s.is_empty() { return Err(ParseError::Empty); }
    match s.parse::<i64>() {
        Ok(n) if n >= 0 => Ok(n),
        Ok(n) => Err(ParseError::Negative(n)),
        Err(_) => Err(ParseError::NotANumber(s.into())),
    }
}

fn add_two(a: &str, b: &str) -> Result<i64, ParseError> {
    Ok(parse_positive(a)? + parse_positive(b)?)
}

fn main() {
    for s in ["42", "", "abc", "-5"] {
        println!("{:?} -> {:?}", s, parse_positive(s));
    }
    println!("{:?}", add_two("10", "20"));   // Ok(30)
    println!("{:?}", add_two("10", "abc"));  // Err(NotANumber)
}
