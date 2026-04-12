// Solution 3: Trait Transforms
use std::marker::PhantomData;

struct IntCol;
struct StrCol;
struct FloatCol;

struct Column<T> {
    name: String,
    data: Vec<String>,
    _type: PhantomData<T>,
}

impl<T> Column<T> {
    fn new(name: &str, data: Vec<&str>) -> Self {
        Column {
            name: name.to_string(),
            data: data.iter().map(|s| s.to_string()).collect(),
            _type: PhantomData,
        }
    }
}

trait Numeric {}
impl Numeric for IntCol {}
impl Numeric for FloatCol {}

impl<T: Numeric> Column<T> {
    fn sum(&self) -> f64 {
        self.data.iter().filter_map(|s| s.parse::<f64>().ok()).sum()
    }
}

impl Column<StrCol> {
    fn uppercase(&self) -> Column<StrCol> {
        Column {
            name: self.name.clone(),
            data: self.data.iter().map(|s| s.to_uppercase()).collect(),
            _type: PhantomData,
        }
    }
}

impl Column<IntCol> {
    fn cast_to_float(&self) -> Column<FloatCol> {
        Column {
            name: self.name.clone(),
            data: self.data.clone(),
            _type: PhantomData,
        }
    }
}

fn main() {
    let amount = Column::<IntCol>::new("amount", vec!["100", "250", "50"]);
    let name = Column::<StrCol>::new("customer", vec!["alice", "bob"]);
    let price = Column::<FloatCol>::new("price", vec!["9.99", "24.50"]);

    println!("Amount sum: {}", amount.sum());
    println!("Price sum: {}", price.sum());

    let upper = name.uppercase();
    println!("Uppercase: {:?}", upper.data);

    let as_float = amount.cast_to_float();
    println!("Cast sum: {}", as_float.sum());

    // Won't compile:
    // name.sum();           // StrCol: Numeric is not satisfied
    // amount.uppercase();   // no method `uppercase` on Column<IntCol>
    // name.cast_to_float(); // no method `cast_to_float` on Column<StrCol>
}
