use std::io::{stdin, Read};
use std::str::Chars;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace();

    let a = input.next().unwrap().chars();
    let b = input.next().unwrap().chars();
    let const_mod: usize = 20_150_523;
    let mut dp_0: Vec<Vec<usize>> = vec![vec![0; 2]; a.clone().count().max(b.clone().count())+1];
    let mut dp_1: Vec<Vec<usize>> = vec![vec![0; 3]; a.clone().count().max(b.clone().count())+1];

    dp_0[0][1] = 1;
    let mut t = 10;

    for i in 1..dp_0.len() {
        dp_0[i][0] = (dp_0[i-1][0] * 7 + dp_0[i-1][1] * 3) % const_mod;
        dp_0[i][1] = t;

        t = (t * 10) % const_mod;
    }

    dp_1[1] = vec![1, 3, 3];

    for i in 2..dp_1.len() {
        dp_1[i][0] = (dp_1[i-1][0] * dp_1[1][0] + dp_1[i-1][1] * dp_1[1][2] + dp_1[i-1][2] * dp_1[1][1]) % const_mod;
        dp_1[i][1] = (dp_1[i-1][0] * dp_1[1][1] + dp_1[i-1][1] * dp_1[1][0] + dp_1[i-1][2] * dp_1[1][2]) % const_mod;
        dp_1[i][2] = (dp_1[i-1][0] * dp_1[1][2] + dp_1[i-1][1] * dp_1[1][1] + dp_1[i-1][2] * dp_1[1][0]) % const_mod;
    }

    let a_res = get_res(a, &dp_0, &dp_1, const_mod, false);
    let b_res = get_res(b, &dp_0, &dp_1, const_mod, true);

    println!("{}", (const_mod + b_res - a_res) % const_mod);
}

fn get_res(s: Chars<'_>, dp_0: &Vec<Vec<usize>>, dp_1: &Vec<Vec<usize>>, const_mod: usize, include: bool) -> usize {
    let mut res = 0;
    let mut tmp = 0;
    let mut switch = false;
    let mut remain = 0;
    let s_len = s.clone().count();

    for (i, x) in s.enumerate() {
        let rev_i = s_len - i - 1;
        let x_digit = x.to_digit(10).unwrap() as usize;

        if switch {
            tmp = (tmp * 10 + x_digit) % const_mod;
            if rev_i == 0 && include {
                tmp += 1;
            }
        } else {
            if rev_i == 0 {
                for j in 0..(if include {x_digit+1} else {x_digit}) {
                    res += if (remain + j) % 3 == 0 || (j % 3 == 0 && j != 0) { 1 } else { 0 };
                }
            } else {
                for j in 0..x_digit {
                    if j == 0 || j % 3 != 0 {
                        res = (res + dp_1[rev_i][(remain + j) % 3]) % const_mod;
                        res = (res + dp_0[rev_i][0]) % const_mod;
                    } else {
                        res = (res + dp_0[rev_i][1]) % const_mod;
                    }
                }
            }
        }

        remain = (remain + x_digit) % 3;

        if x == '3' || x == '6' || x == '9' {
            switch = true;
        }
    }

    (res + tmp) % const_mod
}