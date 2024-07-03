mod failure_detector;
mod utils;
mod node;
mod peers;

use indexmap::IndexMap;

#[derive(Clone)]
enum Value {
    String(String),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Integers(Vec<i64>),
    Floats(Vec<f64>),
}

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut x = IndexMap::<&str, u64>::new();
    x.insert("a", 454);
    x.insert("a", 11);
    x.insert("b", 22);
    x.insert("c", 33);

    let mut c = 0;
    while c < 100 {
        let (k,v) = x.get_index(c % x.len()).unwrap();
        println!("{}: {} => {}", c, k, v);
        c += 1;
        if c == 4 {
            x.shift_remove("b");
        }
        if c == 10 {
            x.insert("d", 44);
        }
        if c > 100 {
            break;
        }
    }
}
