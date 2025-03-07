use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();

    let _t = input.next().unwrap();
    let num = input;

    for n in num {
        let result = math(n);
        writeln!(output, "{result}").unwrap();
    }


    print!("{output}");
}

fn math(n: usize) -> usize {
    let mut output: usize = 0;

    let mut i: usize = 2;

    while n > i * (i-1) / 2 {
        let m = i * (i-1) / 2;
        if (n - m) % i == 0 {
            output += 1;
        }
        i += 1;
    }

    output
}