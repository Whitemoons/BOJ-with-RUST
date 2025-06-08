use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap());

    let n = input.next().unwrap();
    let const_mod: usize = 1_000_000;
    let mut dp: Vec<usize> = vec![0; n+1];
    dp[0] = 1;
    dp[1] = 1;

    if n == 1 {
        println!("1");
        return;
    }

    for i in 2..=n {
        let mut c = 1;
        for j in 0..i {
            if j % 2 == 0 {
                let add = ((dp[j] as u128 * dp[i-j-1] as u128) % const_mod as u128 * (c % const_mod as u128)) % const_mod as u128;
                dp[i] = (dp[i] + add as usize) % const_mod;
            }
            c = c * (i-j-1) as u128 / (j+1) as u128;
        }
    }

    println!("{}", (dp[n] * 2) % const_mod);
}