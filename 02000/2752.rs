use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();
    let c: usize = iter.next().unwrap().parse().unwrap();

    if a > b {
        if b > c {
            println!("{} {} {}", c, b, a);
        } else {
            if a > c {
                println!("{} {} {}", b, c, a);
            } else {
                println!("{} {} {}", b, a, c);
            }
        }
    } else{
        if a > c {
            println!("{} {} {}", c, a, b);
        } else {
            if b > c {
                println!("{} {} {}", a, c, b);
            } else {
                println!("{} {} {}", a, b, c);
            }
        }
    }
}