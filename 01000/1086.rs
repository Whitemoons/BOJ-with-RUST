use std::{io::{stdin, Read}, vec};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let arr: Vec<_> = input.collect();
    let (last, nums) = arr.split_last().unwrap();
    let k = last.parse::<usize>().unwrap();

    let mut remain: Vec<usize> = vec![0; n];
    let mut len: Vec<usize> = vec![0; n];

    for (idx, &num) in nums.iter().enumerate() {
        len[idx] = num.len();
        for digit in num.chars() {
            remain[idx] = (remain[idx] * 10 + digit.to_digit(10).unwrap() as usize) % k;
        }   
    }

    let mut dp = vec![vec![0u64; k]; 1<<n];
    let mut by_popcount = vec![vec![]; n+1];

    for used in 1usize..(1<<n) {
        let cnt = used.count_ones() as usize;
        by_popcount[cnt].push(used);
    }

    for (idx, bit_order) in by_popcount.iter().enumerate() {
        if idx == 1 {
            for &num in bit_order {
                for first in 0..n {
                    if num & (1<<first) != 0 {
                        let rem = remain[first];
                        dp[num][rem % k] = 1;
                    }
                }
            }
        } else {
            for &num in bit_order {
                for first in 0..n {
                    if num & (1<<first) != 0 {
                        let sub_num = num - (1<<first);
                        let m = mul(n, sub_num, &len);
                        let rem = remain[first] * mod_pow(10, m, k);
                        for i in 0..k {
                            dp[num][(i+rem) % k] += dp[sub_num][i];
                        }
                    }
                }
            }
        }
    }

    let (p, q) = cal_ans(&dp.last().unwrap());

    println!("{}/{}", p, q);
}

fn mod_pow(base: usize, exp: usize, modulus: usize) -> usize {
    if exp == 0 {
        1
    } else if exp % 2 == 1 {
        base % modulus * mod_pow(base * base % modulus, exp / 2, modulus) % modulus
    } else {
        mod_pow(base * base % modulus, exp / 2, modulus)
    }
}

fn mul(n: usize, num: usize, len: &Vec<usize>) -> usize {
    let mut output = 0;
    for i in 0..n {
        if num & (1<<i) != 0 {
            output += len[i];
        }
    } 
    output
}

fn cal_ans(arr: &Vec<u64>) -> (u64, u64) {
    let p_t = arr[0];
    let q_t = arr.iter().sum();
    let gcd = gcd(p_t, q_t);
    (p_t / gcd, q_t / gcd)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}