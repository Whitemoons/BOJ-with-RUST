use std::io::{stdin, Read};

struct SegmentTree {
    n: usize,
    tree: Vec<usize>,
}

impl SegmentTree {
    // 초기화: 입력 배열로부터 트리 빌드
    fn new(arr: &[usize]) -> Self {
        let n = arr.len();
        let tree = vec![0; n * 4]; // 넉넉하게 4배 크기
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
            self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
        }
    }

    // 구간합 쿼리: [l, r] 구간의 합
    fn query(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> usize {
        if r < start || end < l {
            // 완전히 겹치지 않는 경우
            0
        } else if l <= start && end <= r {
            // 완전히 포함된 경우
            self.tree[node]
        } else {
            let mid = (start + end) / 2;
            let left_sum = self.query(node * 2, start, mid, l, r);
            let right_sum = self.query(node * 2 + 1, mid + 1, end, l, r);
            left_sum + right_sum
        }
    }

    // 점 업데이트: idx 번째 원소를 val로 업데이트
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
            self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
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

    let mut seg_tree = SegmentTree::new(&vec![0; n]);

    let mut output_list = Vec::new();

    for _ in 0..m {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let c: usize = iter.next().unwrap().parse().unwrap();
        if a == 1 {
            seg_tree.point_update(b-1, c);
        } else if a == 0 {

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