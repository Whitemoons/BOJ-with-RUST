use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace();

    let modulus = 10_007;

    let seq: Vec<char> = input.next().unwrap().chars().collect();
   
    println!("{}", dp(&seq, modulus));
}

fn dp(seq: &Vec<char>, modulus: usize) -> usize {
    let n = seq.len();
    let mut dp: Vec<Vec<usize>> = (0..n).map(|i| vec![0; n - i]).collect();

    for i in 0..n {
        for j in 0..(n-i) {
            if i == 0 {
                dp[i][j] = 1;
            } else if i == 1 {
                dp[i][j] = (dp[i-1][j] + dp[i-1][j+1] + if seq[j] == seq[j+1] {1} else {0}) % modulus;
            } else {
                dp[i][j] = (dp[i-1][j] + dp[i-1][j+1] + if seq[j] == seq[j+i] {1} else {modulus - dp[i-2][j+1]}) % modulus;
            }
        }
    }

    dp[n-1][0]
}