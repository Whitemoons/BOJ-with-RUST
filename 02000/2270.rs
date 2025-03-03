use std::io::{stdin, Read};
use std::fmt::Write;
use std::iter::Peekable;
use std::vec;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();

    let n = input.next().unwrap();
    
    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let c = input.next().unwrap();

    let input_vec: Vec<usize> = input.collect();

    let a_disk = input_vec[..a].iter().copied().peekable();
    let b_disk = input_vec[a..a+b].iter().copied().peekable();
    let c_disk = input_vec[a+b..a+b+c].iter().copied().peekable();
    
    let mut disk: Vec<Peekable<_>> = vec![a_disk, b_disk, c_disk];

    let result = math(n, &mut disk);

    writeln!(output, "{}", result.0).unwrap();
    writeln!(output, "{}", result.1).unwrap();

    print!("{output}");
}

fn math(n: usize, disk: &mut Vec<Peekable<impl Iterator<Item = usize>>>) -> (usize, usize) {
    let max_idx = max_3(
        *disk[0].peek().unwrap_or(&0),
        *disk[1].peek().unwrap_or(&0),
        *disk[2].peek().unwrap_or(&0)
    );
    
    disk[max_idx].next();

    (max_idx+1, recur(n-1, max_idx, disk))
}

fn recur<T: Iterator<Item = usize>>(
    n: usize,
    to: usize,
    disk: &mut Vec<Peekable<T>>, 
) -> usize {
    if n == 0 {
        return 0;
    }
    const MOD: usize = 1_000_000;

    let max_idx = max_3(
        *disk[0].peek().unwrap_or(&0),
        *disk[1].peek().unwrap_or(&0),
        *disk[2].peek().unwrap_or(&0)
    );

    disk[max_idx].next();

    let sub = (0..=2).filter(|&x| x != max_idx && x != to).next().unwrap();

    if to == max_idx {
        recur(n-1, to, disk)
    } else {
        (pow_2(n-1) + recur(n-1, sub, disk)) % MOD
    }
}

fn max_3(a: usize, b: usize, c: usize) -> usize {
    if a >= b {
        if a >= c {
            0
        } else {
            2
        }
    } else {
        if b >= c {
            1
        } else {
            2
        }
    }
}

fn pow_2(n:usize) -> usize {
    const MOD: u128 = 1_000_000;
    let mut output: u128 = 1;
    let mut m = n;
    let mut t: u128 = 2;

    while m >= 1 {
        if m % 2 == 1 {
            output = (output * t) % MOD;
        } 
        m /= 2;
        t = (t * t) % MOD;
    }

    output as usize
}