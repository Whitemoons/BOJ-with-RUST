use std::io::{stdin, Read};

#[derive(Debug)]
struct Node {
    value: usize,
    same: usize,
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();


    let mut iter = buffer.split_ascii_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut ans = 0;
    let mut stack: Vec<Node> = Vec::new();
    let mut same = 1;

    for _ in 0..n {
        let k: usize = iter.next().unwrap().parse().unwrap();
        while let Some(top) = stack.last() {
            if top.value > k {
                ans += 1;
                break;
            } else if top.value == k {
                ans += top.same;
                same = top.same + 1;
                stack.pop();
            } else {
                ans += top.same;
                stack.pop();
            }
        }
        stack.push(Node { value: k, same: same });
        same = 1;
    }

    println!("{}", ans);
}