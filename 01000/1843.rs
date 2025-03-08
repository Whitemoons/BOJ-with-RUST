use std::io::{stdin, Read};
use std::fmt::Write;
use std::vec;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();

    let n = input.next().unwrap();

    let result_a = a(n);
    let result_b = b(n);
    let result_c = c(n);
    
    writeln!(output, "{}", result_a).unwrap();
    writeln!(output, "{}", result_b).unwrap();
    writeln!(output, "{}", result_c).unwrap();

    print!("{output}");
}

fn a(n: usize) -> usize {
    (3..=n).map(|x| (x - 1) / 2).sum::<usize>()
}

fn b(n: usize) -> usize {
    let mut output = 0;

    let mut divisor: Vec<usize> = vec![];
    
    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            divisor.push(i);
            if n != i * i {
                divisor.push(n / i);
            }
        }
        i += 1;
    }

    divisor.sort();

    for z in 0..divisor.len() {
        for y in 0..z {
            for x in 0..=y {
                if divisor[x] + divisor[y] == divisor[z] {
                    output += 1;
                }
            }
        }
    }

    output
}

fn c(n: usize) -> usize {
    let mut output = 0;

    let prime_list = get_prime(n);

    for z in 2..prime_list.len() {
        if is_prime(prime_list[z] - 2) {
            output += 1;
        }
    }

    output
}

fn is_prime(n: usize) -> bool {
    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;

    let mut is_prime = true;

    while i * i <= n {
        if n % i == 0 {
            is_prime = false;
            break;
        }

        i += 2;
    }

    is_prime
}

fn get_prime(n: usize) -> Vec<usize> {
    let mut output: Vec<usize> = vec![2] ;

    for i in (3..=n).step_by(2) {
        let mut is_prime: bool = true;
        for &j in output.iter() {
            if j * j > i {
                break;
            }
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            output.push(i);
        }
    }

    output
}