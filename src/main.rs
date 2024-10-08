mod failure_detector;
mod utils;
mod node;
mod peers;
mod gossip;
mod value;

use indexmap::IndexMap;


fn main() {
    println!("Hello, world!");

    let mut x = IndexMap::<&str, u64>::new();
    x.insert("a", 454);
    x.insert("a", 11);
    x.insert("b", 22);
    x.insert("c", 33);

    let mut c = 0;
    while c < 20 {
        let (k,v) = x.get_index(c % x.len()).unwrap();
        println!("{}: {} => {}", c, k, v);
        c += 1;
        if c == 4 {
            x.shift_remove("b");
        }
        if c == 10 {
            x.insert("d", 44);
        }
    }

    let mut rng = utils::rng(None);
    for _ in 0..10 {
        println!("{}", rng.rand_u64());
    }
    println!("x");
    for _ in 0..10 {
        println!("{}", rng.rand_u64());
    }

    let mut aa = (0..100).collect();
    utils::rand::shuffle(&mut rng, &mut aa, usize::MAX);
    println!("{:?}", aa);

    aa = (0..100).collect();
    utils::rand::shuffle(&mut rng, &mut aa, 2);
    println!("{:?}", aa);
}
