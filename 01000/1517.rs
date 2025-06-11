use std::io::{stdin, Read};

struct MergeCnt {
    cnt: u128,
    arr: Vec<usize>,
    tmp: Vec<usize>
}

impl MergeCnt {
    fn new(n: usize, arr: Vec<usize>) -> Self {
        let cnt = 0;
        let tmp = vec![0; n];
        let mut merge = MergeCnt { cnt, arr, tmp };
        merge.merge(0, n-1);
        merge
    }

    fn merge(&mut self, start: usize, end: usize) {
        if start != end {
            let mid = (start + end) / 2;

            self.merge(start, mid);
            self.merge(mid+1, end);

            let mut i = start;
            let mut j = mid+1;
            let mut idx = start;

            while i <= mid && j <= end {
                if self.arr[i] <= self.arr[j] {
                    self.tmp[idx] = self.arr[i];
                    i += 1;
                } else {
                    self.tmp[idx] = self.arr[j];
                    self.cnt += (j - idx) as u128;
                    j += 1;
                }
                idx += 1;
            }

            while i <= mid {
                self.tmp[idx] = self.arr[i];
                i += 1;
                idx += 1;
            }
            while j <= end {
                self.tmp[idx] = self.arr[j];
                self.cnt += (j - idx) as u128;
                j += 1;
                idx += 1;
            }

            self.copy_tmp_to_arr(start, end);
        }
    }

    fn copy_tmp_to_arr(&mut self, start: usize, end: usize) {
        self.arr[start..=end].copy_from_slice(&self.tmp[start..=end]);
    }

    fn ans(&self) -> u128 {
        self.cnt
    }
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let arr = input.collect::<Vec<usize>>();

    let m = MergeCnt::new(n, arr);

    println!("{}", m.ans());
}