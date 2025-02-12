use std::io::stdin;
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let mut output = String::new();

    let n = buffer.trim().parse::<usize>().unwrap();
    const M: usize = 1_000_000_000;

    let mut arr = vec![0; n/2+1];

    arr[0] = 1;
 
    let result = dp(n, M, &mut arr);

    writeln!(output, "{result}").unwrap();
    
    println!("{output}");
}

fn dp(n:usize, m:usize, arr: &mut [usize]) -> usize {
    for i in 1..=n/2 {
        arr[i] = (arr[i-1] + arr[i/2]) % m;
    }
    return arr[n/2];
}