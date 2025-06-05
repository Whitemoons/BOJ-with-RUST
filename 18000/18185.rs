use std::io::{stdin, Read};


fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();
    
    let n = iter.next().unwrap().parse::<usize>().unwrap();

    let mut res = 0;
    let mut input: Vec<usize> = iter.map(|x| x.parse::<usize>().unwrap()).collect();
    input.push(0);
    input.push(0);

    for i in 0..n {
        if input[i+1] > input[i+2] {
            let buy_cnt = input[i].min(input[i+1] - input[i+2]);
            input[i] -= buy_cnt;
            input[i+1] -= buy_cnt;
            res += buy_cnt * 5;
        }

        let buy_cnt = input[i].min(input[i+1]).min(input[i+2]);
        input[i] -= buy_cnt;
        input[i+1] -= buy_cnt;
        input[i+2] -= buy_cnt;
        res += buy_cnt * 7;

        res += 3 * input[i];
        input[i] = 0;
    }


    println!("{}", res);
}