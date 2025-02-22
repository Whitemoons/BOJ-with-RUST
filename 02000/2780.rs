use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();
    
    let n = input.next().unwrap();
    
    let mut password: Vec<[usize ; 7]> = vec![[1 ; 7]];

    for _ in 0..n {
        let length = input.next().unwrap();


        let result = dp(length, &mut password);
        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}

fn dp(n: usize, password: &mut Vec<[usize; 7]>) -> usize {
    const MOD: usize = 1_234_567;
    
    while password.len() < n {
        let last = password.last().unwrap();
        password.push([
            (2 * last[2]) % MOD,
            (2 * (last[2] + last[4])) % MOD,
            (last[0] + last[1] + last[5]) % MOD,
            (2 * last[4] + last[6]) % MOD,
            (last[1] + last[3] + last[5]) % MOD,
            (last[2] + last[4]) % MOD,
            last[3] % MOD,
        ]);
    }
    
    (password[n-1][2] + password[n-1][4] + password[n-1][5] + password[n - 1].iter().sum::<usize>()) % MOD
}