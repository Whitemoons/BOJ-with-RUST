use std::io::{stdin, Read};

fn lower_bound(start: usize, end: usize, target: i32, lis: &Vec<i32>) -> usize {
    let mut start = start;
    let mut end = end;
    let mut mid: usize;
    
    while start < end {
        mid = (start + end) >> 1;
        if lis[mid] < target {
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    start
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n: usize = input.next().unwrap() as usize;
    let mut dp: Vec<(usize, i32)> = vec![(0, 0); n];
    let mut lis: Vec<i32> = Vec::new();

    for (i, a) in input.enumerate() {
        let a = a as i32;
        if lis.is_empty() || lis[lis.len() - 1] < a {
            lis.push(a);
            dp[i] = (lis.len()-1, a);
        } else {
            let idx = lower_bound(0, lis.len(), a, &lis);
            lis[idx] = a;
            dp[i] = (idx, a);
        }
    }

    let mut ans: Vec<i32> = Vec::new();
    let mut cur = lis.len()-1;

    for i in (0..n).rev() {
        if dp[i].0 == cur {
            ans.push(dp[i].1);
            if cur == 0 {
                break;
            }
            cur -= 1;
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().rev().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}