use std::collections::BTreeMap;
use std::io::{stdin, Read};
use std::fmt::Write;
use std::{usize, vec};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();

    let n = input.next().unwrap();

    let mut num = input;

    let result = math(n, &mut num);

    writeln!(output, "{} {}", result.0, result.1).unwrap();

    print!("{output}");
}

fn math(n: usize, num: &mut impl Iterator<Item = usize>) -> (usize, usize) {
    let prime: Vec<usize> = create_prime(1_000_000);
    let mut factor: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    let num_vec: Vec<usize> = num.collect();

    let mut output: (usize, usize) = (1,0);

    for i in 0..num_vec.len() {
        let mut t = num_vec[i];
        for &k in prime.iter() {
            if k > t { break; };
            if t % k == 0 {
                let mut cnt = 0;
                while t % k == 0 {
                    cnt += 1;
                    t /= k;
                }
                match factor.get(&k) {
                    Some(v) => {
                        let mut tmp = v.clone();
                        tmp[i] = cnt;
                        factor.insert(k, tmp);
                    },
                    None => {
                        let mut tmp: Vec<usize> = vec![0 ; n];
                        tmp[i] = cnt;
                        factor.insert(k, tmp);
                    }
                }
            }
        }
    }

    for (&k, v) in &factor {
        let avg = v.iter().sum::<usize>() / n;
        output.0 *= k.pow(avg as u32);
        output.1 += v.iter().filter(|e| e < &&avg).map(|e| avg - e).sum::<usize>();
    }

    output
}

fn create_prime(n: usize) -> Vec<usize> {
    let mut output: Vec<usize> = vec![];
    output.push(2);
    for i in (3..=n).step_by(2) {
        let mut is_p: bool = true;
        for &p in output.iter() {
            if p * p > i { break; }
            if i % p == 0 {
                is_p = false;
                break;
            }
        }
        if is_p {
            output.push(i);
        }
    }

    output
}