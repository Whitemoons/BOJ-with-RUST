use std::io::{stdin, Read};
use std::fmt::Write;
use std::usize;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();

    let n = input.next().unwrap();

    let num: Vec<usize> = input.collect();

    let result = math(n, &num);

    writeln!(output, "{result}").unwrap();

    print!("{output}");
}

fn math(n: usize, num: &Vec<usize>) -> usize {
    let mut output: usize = 0;

    let mut size: Vec<usize> = vec![];

    for &i in num.iter() {
        count(&mut size, i);
    }

    let mut bi = 1;
    for &i in size.iter() {
        output += i * (n-i) * bi;
        bi *= 2;
    }

    output
}

fn count(size: &mut Vec<usize> , n: usize) -> () {
    let mut m = n;

    let mut i = 0;

    while m != 0 {
        if i >= size.len() {
            size.push(0);
        }
        if m % 2 == 1 {
            size[i] += 1;
        }

        i += 1;
        m /= 2;
    }
}