use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: u64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut scoville: Vec<i64> = lines
        .next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    scoville.sort();

    let mut res: i64 = 0;
    const MOD: i64 = 1_000_000_007;

    for (idx, v) in scoville.iter().enumerate() {
        let add = mod_pow(idx as u64, MOD as u64);
        let minus = mod_pow(n - 1 - idx as u64, MOD as u64);
        res = (res + (add - minus) * v) % MOD;
    }
    
    println!("{:?}", res)
}

fn mod_pow(mut exp: u64, modulo: u64) -> i64 {
    let mut base = 2u64;
    let mut result = 1u64;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulo;
        }
        base = base * base % modulo;
        exp /= 2;
    }

    result as i64
}