use std::io::{stdin, Read};
use std::collections::BinaryHeap;
use std::cmp::{Ordering};

#[derive(Eq, PartialEq)]
struct InputCounter {
    w: usize,
    counter: usize,
}

impl Ord for InputCounter {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.w == other.w {
            self.counter.cmp(&other.counter).reverse()
        } else {
            self.w.cmp(&other.w).reverse()
        }
    }
}

impl PartialOrd for InputCounter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct OutputCustomer {
    w: usize,
    counter: usize,
    id: usize,
}

impl Ord for OutputCustomer {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.w == other.w {
            self.counter.cmp(&other.counter)
        } else {
            self.w.cmp(&other.w).reverse()
        }
    }
}
impl PartialOrd for OutputCustomer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}




fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();
    
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let k = iter.next().unwrap().parse::<usize>().unwrap();

    let mut in_heap = BinaryHeap::new();
    let mut out_heap = BinaryHeap::new();

    for counter_num in 0..k {
        in_heap.push(InputCounter { w: 0, counter: counter_num });
    }

    for _ in 0..n {
        let id = iter.next().unwrap().parse::<usize>().unwrap();
        let w = iter.next().unwrap().parse::<usize>().unwrap();

        let Some(InputCounter { w: w_p, counter: counter_num }) = in_heap.pop() else {
            panic!("No counter available");
        };
        in_heap.push(InputCounter { w: w_p + w, counter: counter_num });
        out_heap.push(OutputCustomer { w: w_p + w, counter: counter_num, id: id });
    }

    let mut res: u128 = 0;
    let mut ord = 1;

    while let Some(OutputCustomer { w: _, counter:_, id }) = out_heap.pop() {
        res += ord * id as u128;
        ord += 1;
    }

    println!("{}", res);
}