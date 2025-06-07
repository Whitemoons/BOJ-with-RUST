use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap());

    let n = input.next().unwrap();
    let const_mod: usize = 1_000_000_007;
    let mut a: usize = 2;
    let mut b: usize = 3;

    for _ in 1..n {
        b = (2*a + b + 4) % const_mod;
        a = (2*a + 2) % const_mod;
    }

    println!("{}", b);
}