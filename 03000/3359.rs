use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();

    let n = input.next().unwrap();
    
    let mut square = input;

    let result: usize = dp(n, &mut square);

    writeln!(output, "{result}").unwrap();

    print!("{output}");
}

fn dp(n: usize, square: &mut impl Iterator<Item = usize>) -> usize {
    let mut arr: Vec<(usize, usize)> = vec![];
    
    let mut la = square.next().unwrap();
    let mut lb = square.next().unwrap();
    
    arr.push((la,lb));
    
    for i in 0..n-1 {
        let a = square.next().unwrap();
        let b = square.next().unwrap();
        arr.push((
            (arr[i].0 + difference(lb, b)).max(arr[i].1 + difference(la, b)) + a,
            (arr[i].0 + difference(lb, a)).max(arr[i].1 + difference(la, a)) + b
        ));
        la = a;
        lb = b;
    }
    let last = arr.last().unwrap();
    let output: usize = last.0.max(last.1);

    output
}

fn difference(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}