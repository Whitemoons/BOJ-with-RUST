use std::io::{stdin, Read};
use std::fmt::Write;
use std::vec;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut output = String::new();
    
    let n = input.next().unwrap();
    let m = input.next().unwrap();

    let distance:Vec<usize> = input.collect();

    let result = dp(n, m, &distance);

    writeln!(output, "{result}").unwrap();
    print!("{output}");
}

fn dp(n: usize, m: usize, distance: &Vec<usize>) -> usize {
    let mut arr: Vec<(usize, usize)> = vec![(0,0) ; n];
    arr[0] = (0, m-distance[0]);
    for i in 1..distance.len() {
        let mut len = distance[i];
        let mut min:Option<(usize, usize)> = None;
        let mut j = i;
        while len <= m && j >= 1 {
            let mut tmp: (usize, usize) = arr[j-1];
            if len < tmp.1 {
                tmp.1 -= len+1;
            } else {
                tmp.0 = cal_score(&tmp);
                tmp.1 = m - len;
            }
            min = match min {
                None => Some(tmp),
                Some(a) => if i == distance.len()-1 {
                    if tmp.0 < a.0 {
                        Some(tmp)
                    } else {
                        min
                    }
                } else {
                    if cal_score(&tmp) < cal_score(&a) || ( cal_score(&tmp) == cal_score(&a) && tmp.1 > a.1){ 
                        Some(tmp) 
                    } else { 
                        min 
                    }
                }
            };
            j -= 1;
            len += distance[j]+1;
        };
        arr[i] = min.unwrap();
    }
    return arr.last().unwrap().0
}

fn cal_score(a: &(usize, usize)) -> usize {
    a.0 + a.1 * a.1
}