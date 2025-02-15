use std::io::stdin;
use std::fmt::Write;
use std::{usize, vec};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    
    let mut output = String::new();
    
    let n = buffer.trim().parse::<usize>().unwrap();
    
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();

    let number: Vec<i32> = buffer.split_ascii_whitespace()
                                    .map(|x| x.parse().unwrap())
                                    .collect();

    let result = dp(n, &number);

    writeln!(output, "{result}").unwrap();
    
    print!("{output}");
}

fn dp(n: usize, number: &[i32]) -> usize {
    let mut arr: Vec<Vec<usize>> = vec![ vec![] ; n];
    let mut output: Vec<usize> = vec![0 ; n];

    for i in (0..number.len()).rev() {
        if arr[i].len() == 0 {
            output[i] = 0;
        } else {
            arr[i].sort();
            let n = arr[i].len();
            output[i] = arr[i].iter().enumerate().map(|(idx, x)| n-idx+*x).max().unwrap_or(0);
        }
        if i == 0 {break;}
        arr[number[i] as usize].push(output[i]);
    }

    return output[0];
}