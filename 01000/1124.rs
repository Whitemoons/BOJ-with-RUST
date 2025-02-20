use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();
    
    let n = input.next().unwrap();
    let m = input.next().unwrap();

    let prime: Vec<usize> = (2..=100_000).filter(|&n| is_prime(n)).collect();

    let result:usize = (n..=m).filter(|&n| is_under_prime(n, &prime)).count();

    writeln!(output, "{result}").unwrap();
    print!("{output}");
}

fn is_under_prime(n: usize, prime: &Vec<usize>) -> bool {
    let mut t = n;
    let mut cnt = 0;
    
    for &p in prime {
        while t % p == 0 {
            cnt += 1;
            t /= p;
        }
        if t == 1 {
            break;
        }
    }

    prime.contains(&cnt)
}

fn is_prime(n: usize) -> bool {
    if n == 1 {
        false
    } else if n == 2 || n == 3 {
        true
    } else if n % 2 == 0 {
        false
    } else {
        !(3..=((n as f64).sqrt() as usize)).step_by(2).any(|m| n % m == 0)
    }
}