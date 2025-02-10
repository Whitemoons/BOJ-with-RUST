use std::io::stdin;
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<u64>);
    
    let a = input.next().unwrap();
    let b = input.next().unwrap();

    let mut output = String::new();

    let result = "1".repeat(euclidean(a, b) as usize);
    writeln!(output, "{result}").unwrap();

    println!("{output}");
}

fn euclidean(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return a + b
    } else {
        return euclidean(b, a % b);
    }
}