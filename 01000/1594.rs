use std::io::stdin;
use std::fmt::Write;
use std::vec;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let mut output = String::new();

    let number = buffer.trim();

    let mut arr = vec![ (0,0) ; number.len()];

    arr[1].0 = check_score(&number[0..=1]);
    if number.len() > 2 {
        arr[2].0 = check_score(&number[0..=2]);
    }
    if number.len() > 3 {
        arr[3].0 = arr[1].0 + check_score(&number[2..=3]);
        arr[3].1 = 1;
    }
 
    let result = dp(number, &mut arr);

    writeln!(output, "{}", make_tel(number, result)).unwrap();
    
    print!("{output}");
}

fn dp(number:&str, arr: &mut [(usize, usize)]) -> Vec<usize> {
    for i in 4..=number.len()-1 {
        let two = arr[i-2].0 + check_score(&number[i-1..=i]);
        let three = arr[i-3].0 + check_score(&number[i-2..=i]);
        if two > three {
            arr[i] = (two, i-2);
        } else if two < three {
            arr[i] = (three, i-3);
        } else {
            let two_check = return_dash_position(i-2, arr);
            let three_check = return_dash_position(i-3, arr);
            let mut pos = 0;
            loop {
                if two_check[pos] < three_check[pos] {
                    arr[i] = (two, i-2);
                    break;
                } else if two_check[pos] > three_check[pos] {
                    arr[i] = (three, i-3);
                    break;
                } else {
                    pos += 1;
                }
            }
        }
    }
    let output = return_dash_position(number.len()-1, arr);
    return output;
}

fn check_score(number: &str) -> usize {
    let chars: Vec<char> = number.chars().collect();

    if number.len() == 2 {
        return if chars[0] == chars[1] { 2 } else { 0 };
    }

    if chars[0] == chars[1] && chars[1] == chars[2] {
        2
    } else if chars[0] == chars[1] || chars[1] == chars[2] || chars[2] == chars[0] {
        1
    } else {
        0
    }
}

fn return_dash_position(start: usize, arr: &mut [(usize, usize)]) -> Vec<usize> {
    let mut output: Vec<usize> = vec![];
    let mut current = start;
    while current != 0 {
        output.push(current);
        current = arr[current].1;
    };
    output.push(0);
    output.reverse();
    return output;
}

fn make_tel(number: &str, pos: Vec<usize>) -> String {
    let mut output = String::from(&number[pos[0]..=pos[1]]);
    for i in 1..pos.len()-1 {
        output.push('-');
        output.push_str(&number[pos[i]+1..=pos[i+1]]);
    };
    return output;
}