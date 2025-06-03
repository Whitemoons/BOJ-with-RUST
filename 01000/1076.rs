use std::io::{stdin, Read};

fn str_to_num (str: &str) -> usize {
    match str {
        "black" => 0,
        "brown" => 1,
        "red" => 2,
        "orange" => 3,
        "yellow" => 4,
        "green" => 5,
        "blue" => 6,
        "violet" => 7,
        "grey" => 8,
        "white" => 9,
        _ => 0,
    }
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();
    
    let value = (str_to_num(iter.next().unwrap()) * 10 + str_to_num(iter.next().unwrap())) as usize;
    let multiplier = 10_usize.pow(str_to_num(iter.next().unwrap()) as u32);

    println!("{}", value * multiplier);
}