use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<u128>);

    let mut output = String::new();

    let m = input.next().unwrap();
    let a = input.next().unwrap();
    let c = input.next().unwrap();
    let x_0 = input.next().unwrap();
    let n = input.next().unwrap();
    let g = input.next().unwrap();
    
    let x_n = math(m, a, c, x_0, n);

    let result: u128 = x_n % g;

    writeln!(output, "{result}").unwrap();

    print!("{output}");
}

fn math(m: u128, a: u128, c: u128, x_0: u128, n: u128) -> u128 {
    let mut output: u128 = x_0;
    let mut t = n;

    let mut at = a;
    let mut bt = 1;
    while t >= 1 {
        if t % 2 == 1 {
            output = (at * output + c * bt) % m;
        }

        bt = ((at + 1) * bt) % m;
        at = (at * at) % m;

        t /= 2;
    };

    output
}