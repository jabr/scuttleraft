mod failure_detector;
mod utils;
mod node;

enum Value {
    String(String),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Integers(Vec<i64>),
    Floats(Vec<f64>),
}

fn main() {
    println!("Hello, world!");
}
