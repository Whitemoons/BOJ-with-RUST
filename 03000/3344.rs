use std::io::{stdin, Read};
use std::fmt::Write; 

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let n = buffer.trim().parse::<usize>().unwrap();

    let mut output = String::new();

    let result = f(n);

    write!(output, "{}", result).unwrap();

    print!("{output}");
}

fn f(n: usize) -> String {
    let mut odds: Vec<usize> = (6..=n).filter(|x| x % 2 == 1).collect();
    let mut evens: Vec<usize> = (3..=n).filter(|x| x % 2 == 0).collect();

    if n == 2 || n == 3 {
        return String::new();
    }

    if n % 6 == 2 {
        odds.insert(0, 1);
        odds.insert(0, 3);
        odds.push(5);
        evens.insert(0, 2);
    } else if n % 6 == 3 {
        odds.insert(0,5);
        odds.push(1);
        odds.push(3);
        evens.push(2);
    } else {
        odds.insert(0,5);
        odds.insert(0,3);
        odds.insert(0,1);
        evens.insert(0, 2);
    }

    let odds_str = odds.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");

    let evens_str = evens.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    format!("{}\n{}", evens_str, odds_str)
}