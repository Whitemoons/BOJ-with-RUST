use std::io::{stdin, Read};
use std::usize;

const MOD: usize = 10_007;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output: usize = 0;
    
    let n = input.next().unwrap();

    if n < 4 {
        output = 0;
    } else if n >= 40 {
        output = comb(52, 52 - n);
    } else {
        let mut i = 1;
        while 4 * i <= n {
            let sign = if i & 1 == 1 { 1 } else { MOD - 1 };
            output = (output + sign * (comb(13, i) % MOD) * (comb(52 - 4 * i, n - 4 * i) % MOD)) % MOD;
            i += 1;
        }
    }

    print!("{}", output.to_string());
}

fn comb(n: usize, r: usize) -> usize {
    if r == 0 {
        return 1;
    }

    let q: usize = ((n-r+1)..=n).fold(1, |acc, x| (acc * x) % MOD);
    let p: usize = (1..=r).fold(1, |acc, x| (acc * x) % MOD);
    let p_inv = mod_inverse(p);

    (q * p_inv) % MOD
}

fn mod_inverse(n: usize,) -> usize {
    let mut e: usize = MOD - 2;
    let mut a: usize = n;
    let mut result = 1;

    while e > 0 {
        if e & 1 == 1 {
            result = (result * a) % MOD;
        }
        a = (a * a) % MOD;
        e = e >> 1;
    }

    result
}