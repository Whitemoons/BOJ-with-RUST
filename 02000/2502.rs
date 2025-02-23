use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();
    
    let d = input.next().unwrap();
    let k = input.next().unwrap();

    let p: (usize, usize) = dp(d);
    let result = bf(p, k);

    writeln!(output, "{}", result.0).unwrap();
    writeln!(output, "{}", result.1).unwrap();

    print!("{output}");
}

fn dp(d: usize) -> (usize, usize) {
    let mut a_p: Vec<usize> = vec![1, 0];
    let mut b_p: Vec<usize> = vec![0, 1];
    for i in 2..d {
        a_p.push(a_p[i-1] + a_p[i-2]);
        b_p.push(b_p[i-1] + b_p[i-2]);
    }
    
    (a_p[d-1], b_p[d-1])
}

fn bf(p: (usize, usize), k: usize) -> (usize, usize){
    let mut t = 1;
    while p.0 + t * p.1 <= k {
        if (k - t * p.1) % p.0 == 0 && (k - t * p.1) / p.0 <= t {
            break;
        } else {
            t += 1;
        };
    };
    
    ((k - t * p.1) / p.0, t)
}