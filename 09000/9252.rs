use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();

    let a = lines.next().unwrap();
    let b = lines.next().unwrap();

    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let (len, b_mat) = dp(&a, &b);

    if len == 0 {
        println!("{}", len);
    } else {
        println!("{}", len);
        let ans = backtrack(&a, &b, &b_mat);
        println!("{}", ans);
    }
}

fn dp(a: &[char], b: &[char]) -> (usize, Vec<Vec<usize>>) {
    let mut c_mat: Vec<Vec<usize>> = vec![vec![0; b.len() + 1]; a.len() + 1];
    let mut b_mat: Vec<Vec<usize>> = vec![vec![0; b.len() + 1]; a.len() + 1];

    for i in 1..=a.len() {
        for j in 1..=b.len() {
            if a[i - 1] == b[j - 1] {
                c_mat[i][j] = c_mat[i - 1][j - 1] + 1;
                b_mat[i][j] = 1;
            } else {
                if c_mat[i - 1][j] >= c_mat[i][j - 1] {
                    c_mat[i][j] = c_mat[i - 1][j];
                    b_mat[i][j] = 2;
                } else {
                    c_mat[i][j] = c_mat[i][j - 1];
                    b_mat[i][j] = 3;
                }
            }
        }
    }

    (c_mat[a.len()][b.len()], b_mat)
}

fn backtrack(a: &[char], b: &[char], b_mat: &[Vec<usize>]) -> String {
    let mut ans = String::new();

    let mut i = a.len();
    let mut j = b.len();

    while i > 0 && j > 0 {
        if b_mat[i][j] == 1 {
            ans.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if b_mat[i][j] == 2 {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    ans.chars().rev().collect()
}