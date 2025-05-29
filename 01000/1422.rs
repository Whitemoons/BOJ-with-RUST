use std::io::{stdin, Read};
use std::cmp::Ordering;
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let k = input.next().unwrap();
    let n = input.next().unwrap();
    let mut li: Vec<usize> = input.collect();
    let add_ele = vec![find_max_lex_among_longest(&li); n-k];

    li.extend(add_ele);

    li.sort_by(|a, b| compare_lexicographically(b, a));

    let mut output = String::new();
    for num in li {
        write!(&mut output, "{}", num).unwrap();
    }

    println!("{}", output);
}

fn num_to_buf(mut n: usize, buf: &mut [u8; 20]) -> &[u8] {
    let mut i = 20;
    if n == 0 {
        i -= 1;
        buf[i] = b'0';
    } else {
        while n > 0 {
            i -= 1;
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
        }
    }
    &buf[i..]
}

fn compare_lexicographically(a: &usize, b: &usize) -> Ordering {
    let mut abuf = [0u8; 20];
    let mut bbuf = [0u8; 20];

    let a_str = num_to_buf(*a, &mut abuf);
    let b_str = num_to_buf(*b, &mut bbuf);

    let ab_iter = a_str.iter().chain(b_str.iter());
    let ba_iter = b_str.iter().chain(a_str.iter());

    for (x, y) in ab_iter.zip(ba_iter) {
        match x.cmp(y) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }
    Ordering::Equal
}

fn num_digits(mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}

fn find_max_lex_among_longest(nums: &[usize]) -> usize {
    let mut max_len = 0;
    let mut candidates = Vec::new();

    for &num in nums {
        let len = num_digits(num);
        if len > max_len {
            max_len = len;
            candidates.clear();
            candidates.push(num);
        } else if len == max_len {
            candidates.push(num);
        }
    }

    candidates.into_iter().max_by(|a, b| compare_lexicographically(a, b)).unwrap()
}