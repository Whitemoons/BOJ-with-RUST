use std::io::{stdin, Read};

struct Edge {
    from: usize,
    to: usize,
    cost: usize,
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut iter = buffer.split_ascii_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let a = iter.next().unwrap().parse().unwrap();
        let b = iter.next().unwrap().parse().unwrap();
        let c = iter.next().unwrap().parse().unwrap();
        edges.push(Edge {from: a, to: b, cost: c});
    }

    edges.sort_by(|a, b| a.cost.cmp(&b.cost));

    println!("{}", mst(n, &mut edges.into_iter()));
}

fn mst(n: usize, edges: &mut impl Iterator<Item=Edge>) -> usize {
    let mut parent = (0..n).collect::<Vec<_>>();
    
    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while x != parent[x] {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }

    fn union(parent: &mut [usize], a: usize, b: usize) -> bool {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra == rb {
            false
        } else {
            parent[rb] = ra;
            true
        }
    }

    let mut max_cost = 0;
    let mut res = 0;

    for edge in edges {
        if union(&mut parent, edge.from - 1, edge.to - 1) {
            if edge.cost > max_cost { max_cost = edge.cost };
            res += edge.cost;
        }
    }

    res - max_cost
}