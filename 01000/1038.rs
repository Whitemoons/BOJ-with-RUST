use std::io::stdin;
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    
    let mut output = String::new();
    
    let n = buffer.trim().parse::<i32>().unwrap();

    let result = backtracking(n);

    writeln!(output, "{result}").unwrap();
    
    print!("{output}");
}

fn backtracking(n: i32) -> i64 {
    let mut tmp = 0;
    let mut order: i32 = 10;
    let mut output:i64 = 0;
    loop {
        if order >= 110 { return -1; }
        if order < 10 { return output; }
        let add_some = count((order % 10) as usize, (order / 10 - 1) as usize) as i32;
        if tmp + add_some < n+1 {
            tmp += add_some;
            order += 1;
        } else {
            output = output * 10 + (order % 10)as i64;
            order = (order - 10) / 10 * 10;
        }
    }
}

fn count(first_num: usize, digit: usize) -> usize {
    if first_num < digit { return 0; }
    let demon: usize = (1..=first_num).product();
    let numer1: usize = (1..=digit).product();
    let numer2: usize = (1..=first_num-digit).product();
    return demon / numer1 / numer2
}