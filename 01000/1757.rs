use std::io::{stdin, Read};
use std::fmt::Write;
use std::vec;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();
    
    let n = input.next().unwrap();
    let m = input.next().unwrap();

    let mut distance = input;

    let result = dp(n, m, &mut distance);

    writeln!(output, "{result}").unwrap();
    print!("{output}");
}

fn dp(n: usize, m: usize, distance: &mut impl Iterator<Item = usize>) -> usize {
    let mut arr = vec![vec![0 ; m+1] ; n];
    arr[0][1] = distance.next().unwrap();
    arr[1][0] = arr[0][1];
    for i in 1..n {
        let di = distance.next().unwrap();
        for j in 0..=m {
            if j == 0 {
                arr[i][j] = arr[i-1][j].max(arr[i][j]);
            } else {
                arr[i][j] = arr[i-1][j-1] + di;
            }
            if i+j < n {
                arr[i+j][0] = arr[i+j][0].max(arr[i][j]);
            }
        }
    }
    return arr[n-1][0]
}