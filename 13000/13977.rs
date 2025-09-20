use std::io::{stdin, Read};

const MOD: usize = 1_000_000_007;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let m = input.next().unwrap();

    let mut qs: Vec<(usize, usize)> = Vec::with_capacity(m);
    let mut max_n = 0;
    
    for _ in 0..m {
        let n = input.next().unwrap();
        let mut k = input.next().unwrap();
        if k * 2 >  n {  k = n - k; }

        qs.push((n, k));
        max_n = max_n.max(n);
    }

    let mut fac = vec![1; max_n + 1];
    let mut inv_fac = vec![1; max_n + 1];

    for i in 1..=max_n {
        fac[i] = fac[i - 1] * i % MOD;
    }

    inv_fac[max_n] = mod_pow(fac[max_n], MOD - 2);

    for i in (1..=max_n).rev() {
        inv_fac[i - 1] = inv_fac[i] * i % MOD;
    }

    let mut output = String::with_capacity(m*12);

    for (n, k) in qs {
        let comb = fac[n] * inv_fac[k] % MOD * inv_fac[n - k] % MOD;
        output.push_str(&comb.to_string());
        output.push('\n');
    }

    print!("{}", output);
}

fn mod_pow(mut a: usize, mut e: usize) -> usize {
    let mut r: usize = 1;
    while e > 0 {
        if e & 1 == 1 { r = (r * a) % MOD; }
        a = (a * a) % MOD;
        e >>= 1;
    }
    r
}
