use std::io::{stdin, Read};
use std::fmt::Write;
use std::vec;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<String>);

    let mut output = String::new();

    let bstring = input.next().unwrap();
    
    let pos = gd(&bstring);

    let result = format_string(&bstring, pos);

    writeln!(output, "{result}").unwrap();

    print!("{output}");
}

fn gd(bstring: &str) -> Vec<(usize,usize)> {
    let mut output: Vec<(usize, usize)> = vec![];
    let mut tmp: Vec<(usize, usize)> = vec![];

    for i in 0..bstring.len() {
        output.push((i,i+1));
    }

    while output != tmp {
        tmp = output;
        output = vec![];
        let mut i = 0;
        while i < tmp.len() {
            let mut j = i;
            while j+1 < tmp.len() && bstring[tmp[j].0..tmp[j].1] <= bstring[tmp[j+1].0..tmp[j+1].1] {
                j += 1;
            }
            output.push((tmp[i].0, tmp[j].1));
            i = j + 1;
        }
    }

    output
}

fn format_string(bstring: &str, pos: Vec<(usize, usize)>) -> String {
    let mut output = String::new();
    for (a,b) in pos.iter() {
        output.push('(');
        output.push_str(&bstring[*a..*b]);
        output.push(')');
    };

    output
}