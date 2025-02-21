use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();
    
    let n = input.next().unwrap();
    
    let mut space_move: Vec<usize> = vec![0, 1];

    for _ in 0..n {
        let x = input.next().unwrap();
        let y = input.next().unwrap();


        let result = math(y-x, &mut space_move);
        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}

fn math(n: usize, space_move: &mut Vec<usize>) -> usize {
    let mut i = 1;
    while i < space_move.len() {
        if n <= space_move[i] {
            break;
        } else {
            i += 1;
            if i == space_move.len() {
                let val = 2 * (1..=i/2).sum::<usize>() + (i % 2 == 1).then_some(i/2 + 1).unwrap_or(0);
                space_move.push(val);
            }
        }
    }

    i
}