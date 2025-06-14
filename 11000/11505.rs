use std::io::{stdin, Read};

struct SegmentTree {
    n: usize,
    tree: Vec<usize>,
}

impl SegmentTree {
    fn new(arr: &[usize]) -> Self {
        let n = arr.len();
        let tree = vec![0; n * 4];
        let mut seg = SegmentTree { n, tree };
        seg.build(arr, 1, 0, n - 1);
        seg
    }

    fn build(&mut self, arr: &[usize], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            self.build(arr, node * 2, start, mid);
            self.build(arr, node * 2 + 1, mid + 1, end);
            self.tree[node] = (self.tree[node * 2] * self.tree[node * 2 + 1]) % 1_000_000_007;
        }
    }

    fn query(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> usize {
        if r < start || end < l {
            1
        } else if l <= start && end <= r {
            self.tree[node]
        } else {
            let mid = (start + end) / 2;
            let left_sum = self.query(node * 2, start, mid, l, r);
            let right_sum = self.query(node * 2 + 1, mid + 1, end, l, r);
            (left_sum * right_sum) % 1_000_000_007
        }
    }

    fn update(&mut self, node: usize, start: usize, end: usize, idx: usize, val: usize) {
        if start == end {
            self.tree[node] = val;
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.update(node * 2, start, mid, idx, val);
            } else {
                self.update(node * 2 + 1, mid + 1, end, idx, val);
            }
            self.tree[node] = self.tree[node * 2] * self.tree[node * 2 + 1] % 1_000_000_007;
        }
    }

    fn range_query(&self, l: usize, r: usize) -> usize {
        self.query(1, 0, self.n - 1, l, r)
    }

    fn point_update(&mut self, idx: usize, val: usize) {
        self.update(1, 0, self.n - 1, idx, val);
    }
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut iter = buffer.split_ascii_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut seg_tree = SegmentTree::new(&vec![0; n]);

    let mut output_list = Vec::new();

    for i in 0..n {
        let v: usize = iter.next().unwrap().parse().unwrap();
        seg_tree.point_update(i, v);
    }

    for _ in 0..(m+k) {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let c: usize = iter.next().unwrap().parse().unwrap();
        if a == 1 {
            seg_tree.point_update(b-1, c);
        } else if a == 2 {

            let output = seg_tree.range_query(b.min(c)-1, c.max(b)-1);
            output_list.push(output);
        }
    }

    let ans = output_list.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join("\n");

    println!("{}", ans);
}