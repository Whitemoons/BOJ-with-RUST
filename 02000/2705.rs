use std::io::stdin;
use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let mut output = String::new();

    let n = buffer.trim().parse::<usize>().unwrap();
    let mut arr: [usize; 1000] = [0; 1000];

    for _ in 0..n {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();

        let m = buffer.trim().parse::<usize>().unwrap();
        let result = dp(m, &mut arr);
        
        writeln!(output, "{result}").unwrap();
    }

    println!("{output}");
}

fn dp(n:usize, arr: &mut [usize]) -> usize {
    if n == 0 {
        return 1;
    } else if arr[n-1] != 0 {
        return arr[n-1];
    } else {
        let mut res = 1;
        for i in (0..n-1).rev().step_by(2) {
            res += dp((n-i)/2, arr);
        }
        arr[n-1] = res;
        return arr[n-1];
    }
}