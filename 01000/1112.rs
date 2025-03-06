use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let mut output = String::new();

    let n = input.next().unwrap();
    let b = input.next().unwrap();

    let result = math(n, b);

    writeln!(output, "{result}").unwrap();

    print!("{output}");
}

fn math(n: i64, b: i64) -> String {
    let mut output: String = String::new();

    let mut t = if n < 0 && b > 0 {
        n.abs()
    } else {
        n
    };

    while t != 0 {
        let v: i64 = t % b;
        let digit = if v >= 0 {
            t /= b;
            v as u8
        } else {
            t = (t - v + b) / b;
            (v - b) as u8
        };
        output.push((digit + b'0') as char);
    }

    if n < 0 && b > 0 {
        output.push('-');
    }

    if output.is_empty() {
        output.push('0');
    }

    output.chars().rev().collect::<String>()
}