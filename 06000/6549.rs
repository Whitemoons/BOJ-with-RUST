use std::io::{stdin};

fn get_max_area(arr: &[usize], left: usize, right: usize) -> usize {
    if left == right {
        return arr[left];
    }

    let mid = (left + right) / 2;

    let max_left = get_max_area(arr, left, mid);
    let max_right = get_max_area(arr, mid + 1, right);
    let max_cross = get_max_cross_area(arr, left, right, mid);

    max_left.max(max_right).max(max_cross)
}

fn get_max_cross_area(arr: &[usize], left: usize, right: usize, mid: usize) -> usize {
    let mut lo = mid;
    let mut hi = mid + 1;
    let mut height = arr[lo].min(arr[hi]);
    let mut max_area = height * 2;

    while left < lo || hi < right {
        if hi < right && (lo == left || arr[lo - 1] < arr[hi + 1]) {
            hi += 1;
            height = height.min(arr[hi]);
        } else if lo > left {
            lo -= 1;
            height = height.min(arr[lo]);
        } else {
            break;
        }

        max_area = max_area.max(height * (hi - lo + 1));
    }

    max_area
}

fn main() {
    let mut output_list = Vec::new();

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        if buffer.trim() == "0" {
            break;
        }

        let mut iter = buffer.split_ascii_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let arr: Vec<usize> = iter.map(|x| x.parse().unwrap()).collect();

        let result = get_max_area(&arr, 0, n - 1);
        output_list.push(result);
    }

    let ans = output_list.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);
}