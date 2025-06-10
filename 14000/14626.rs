use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace();

    let a = input.next().unwrap().chars();

    let mut mul = 0;
    let mut sum = 0;

    for (idx, x) in a.enumerate() {
        if x == '*' {
            mul = if idx % 2 == 1 {3} else {1};
        } else {
            sum += x.to_digit(10).unwrap() * (if idx % 2 == 1 {3} else {1});
        }
    }

    for i in 0..10 {
        if (sum + i * mul) % 10 == 0 {
            println!("{i}");
        }
    }
}