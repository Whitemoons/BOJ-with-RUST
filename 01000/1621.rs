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

    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let k = input.next().unwrap();
    let c = input.next().unwrap();

    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();

    let number:Vec<usize> = buffer.split_ascii_whitespace()
                                    .map(|s| s.trim().parse().expect(""))
                                    .collect::<Vec<_>>();

    let mut arr:Vec<(usize,Option<usize>)> = vec![(0,None) ; n];

    arr[0] = if k == 1 && c < number[0] { (c, Some(1)) } else { (number[0], None) };

    let pos = dp(k, c, &number, &mut arr);

    writeln!(output, "{}", arr.last().unwrap().0).unwrap();
    writeln!(output, "{}", pos.len()).unwrap();
    writeln!(output, "{}", pos.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")).unwrap();
    
    print!("{output}");
}

fn dp(k: usize, c: usize, number: &[usize], arr: &mut [(usize, Option<usize>)]) -> Vec<usize> {
    for i in 1..number.len() {
        let sum_one = arr[i-1].0 + number[i];
        let sum_k = if i < k-1 {
            usize::MAX
        } else if i == k-1 {
            c
        } else {
            arr[i-k].0 + c
        };
        if sum_one > sum_k {
            arr[i] = (sum_k, Some(i+2-k));
        } else {
            arr[i] = (sum_one, arr[i-1].1);
        }
    };
    let mut pos = arr.last().unwrap().1;
    let mut output:Vec<usize> = vec![];
    while let Some(p) = pos {
        output.push(p);
        if p==1 { break; }
        pos = arr[p-2].1;
    }
    output.reverse();

    return output;
}