use std::io::{stdin, Read};
use std::cmp::Ordering;
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace();

    let _n = input.next().unwrap();
    let mut li: Vec<&str> = input.collect();

    let is_zero = li.iter().all(|x| x == &"0");

    if is_zero {
        println!("0");
        return
    }

    li.sort_by(|a, b| compare_lexicographically(b, a));

    let mut output = String::new();
    for num in li {
        write!(&mut output, "{}", num).unwrap();
    }

    println!("{}", output);
}

fn compare_lexicographically(a: &str, b: &str) -> Ordering {
    let ab_iter = a.chars().chain(b.chars());
    let ba_iter = b.chars().chain(a.chars());

    for (x, y) in ab_iter.zip(ba_iter) {
        match x.cmp(&y) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }

    Ordering::Equal
}