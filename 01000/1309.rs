use std::io::{stdin, Read};
use std::fmt::Write;
use std::vec;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();

    let n = input.next().unwrap();

    let result = dp(n);

    writeln!(output, "{result}").unwrap();

    print!("{output}");
}

fn dp(n: usize) -> usize {
    const MOD: usize = 9901;
    let mut arr: Vec<(usize, usize)> = vec![(0,0) ; n];
    arr[0] = (1,1);
    for i in 1..n {
        arr[i] = ((arr[i-1].0 + arr[i-1].1 * 2) % MOD, (arr[i-1].0 + arr[i-1].1) % MOD);
    }
    return (arr[n-1].0 + arr[n-1].1 * 2) % MOD;
}