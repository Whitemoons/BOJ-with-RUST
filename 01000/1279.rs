use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();
    
    let m = input.next().unwrap();

    let result = dp((6 * m).saturating_sub(21));

    writeln!(output, "{result}").unwrap();

    print!("{output}");
}

fn dp(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    const MOD: usize = 1_000_000_007;
    let mut dp: [[usize ; 6] ; 6] = [[0 ; 6] ; 6];

    let mut output:usize = 1;
    for i in 1..=n {
        for j in 1..=n.min(6) {
            if j == 1 || i == j {
                dp[i % 6][j - 1] = 1;
            } else if i < j {
                dp[i % 6][j - 1] = dp[(i-1) % 6][j - 2];
            } else {
                dp[i % 6][j - 1] = (dp[(i-1) % 6][j - 2] + dp[(i - j) % 6][j - 1]) % MOD;
            };
        };
        output = (dp[i % 6].iter().sum::<usize>() + output) % MOD;
    };

    (output * 30) % MOD
}