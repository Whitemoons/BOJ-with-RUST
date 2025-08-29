use std::io::{self, BufRead};

fn proceed(p: &Vec<char>, m: usize) -> Vec<usize> {
    let mut proceed: Vec<usize> = vec![0; m];
    let mut k: usize = 0;

    for j in 1..m {
        while k > 0 && p[k] != p[j] {
            k = proceed[k - 1];
        }
        if p[j] == p[k] {
            k += 1;
        }
        proceed[j] = k;
    }
    proceed
}

fn main() {
    let mut lines = io::stdin().lock().lines();

    let t: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let p: Vec<char> = lines.next().unwrap().unwrap().chars().collect();

    let n = t.len();
    let m = p.len();

    let proceed: Vec<usize> = proceed(&p, m);

    let mut ans: Vec<usize> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < n { 
        if t[i] == p[j] {
            i += 1;
            j += 1;
            if j == m {
                ans.push(i-m+1);
                j = proceed[j - 1];
            }
        } else {
            if j != 0 {
                j = proceed[j - 1];
            } else {
                i += 1;
            }
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}