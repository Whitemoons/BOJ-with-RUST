use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();
    let c: usize = iter.next().unwrap().parse().unwrap();

    if a == b && b == c {
        println!("{}", 10000 + a * 1000);
    } else if a == b {
        println!("{}", 1000 + a * 100);
    } else if b == c {
        println!("{}", 1000 + b * 100);
    } else if a == c {
        println!("{}", 1000 + a * 100);
    } else {
        println!("{}", a.max(b).max(c) * 100);
    }
}