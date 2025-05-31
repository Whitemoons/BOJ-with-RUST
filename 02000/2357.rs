use std::io::{stdin, Read};

struct SegmentTree {
    n: usize,
    tree: Vec<(usize, usize)>,
}

impl SegmentTree {
    fn new(arr: &[usize]) -> Self {
        let n = arr.len();
        let tree: Vec<(usize, usize)> = vec![(0, 0); n * 4]; // 넉넉하게 4배 크기
        let mut seg = SegmentTree { n, tree };
        seg.build(arr, 1, 0, n - 1);
        seg
    }

    fn build(&mut self, arr: &[usize], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = (arr[start], arr[end]);
        } else {
            let mid = (start + end) / 2;
            self.build(arr, node * 2, start, mid);
            self.build(arr, node * 2 + 1, mid + 1, end);
            self.tree[node] = (
                self.tree[node * 2].0.min(self.tree[node * 2 + 1].0),
                self.tree[node * 2].1.max(self.tree[node * 2 + 1].1)
            );
        }
    }

    fn query(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> (usize, usize) {
        if r < start || end < l {
            (1_000_000_001, 0)
        } else if l <= start && end <= r {
            self.tree[node]
        } else {
            let mid = (start + end) / 2;
            let left = self.query(node * 2, start, mid, l, r);
            let right = self.query(node * 2 + 1, mid + 1, end, l, r);
            (left.0.min(right.0), left.1.max(right.1))
        }
    }

    fn range_query(&self, l: usize, r: usize) -> (usize, usize) {
        self.query(1, 0, self.n - 1, l, r)
    }
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let _m: usize = iter.next().unwrap().parse().unwrap();

    let mut output_list = Vec::new();
    
    let mut nums: Vec<usize> = iter.map(|x| x.parse().unwrap()).collect();
    let arr = nums.drain(..n).collect::<Vec<_>>();
    
    let seg_tree = SegmentTree::new(&arr);

    for chunk in nums.chunks(2) {
        let (a, b) = (chunk[0], chunk[1]);
        let output = seg_tree.range_query(a.min(b)-1, a.max(b)-1);
        output_list.push(output);
    }

    let ans = output_list.iter()
                        .map(|(min, max)| format!("{} {}", min, max))
                        .collect::<Vec<_>>()
                        .join("\n");

    println!("{}", ans);
}