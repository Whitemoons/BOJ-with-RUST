use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut output = String::new();
    
    let n = input.next().unwrap();

    let mut seq = input;

    let result = match bf(n, &mut seq) {
        Ok(num) => {
            num.to_string()
        },
        Err(ch) => {
            ch.to_string()
        }
    };

    writeln!(output, "{result}").unwrap();
    print!("{output}");
}

fn bf(n: i32, seq: &mut impl Iterator<Item = i32>) -> Result<i32, char> {
    
    if n == 1 {
        return Err('A');
    } else if n == 2 {
        return seq.next().zip(seq.next())
                .filter(|(first, second)| first == second)
                .map(|(first, _)| first)
                .ok_or('A');
    } else {
        let init: Vec<i32> = seq.by_ref().take(3).collect();

        if init[1] == init[0] {
            if init[2] == init[1] {
                return seq.try_fold(init[0], |last_ele, ele| {
                    (last_ele == ele).then_some(ele).map_or(Err('B'), |_| Ok(ele))
                })
            } else {
                return Err('B')
            }
        } else {
            let a:Option<i32> = ((init[2] - init[1]) % (init[1] - init[0]) == 0)
                                    .then(|| (init[2] - init[1]) / (init[1] - init[0]));
            let b:Option<i32> = ((init[1]*init[1] - init[2]*init[0]) % (init[1] - init[0]) == 0)
                                    .then(|| (init[1]*init[1] - init[2]*init[0]) / (init[1] - init[0]));

            return seq.try_fold(*init.last().unwrap(), |last_ele, ele| {
                a.zip(b)
                    .filter(|(el_a, el_b)| last_ele * el_a + el_b == ele)
                    .map_or(Err('B'), |_| Ok(ele))
            })
            .and_then(|ele| a.zip(b).map_or(Err('B'), |(el_a, el_b)| Ok(ele * el_a + el_b)));
        }
    }
}