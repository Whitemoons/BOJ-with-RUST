use std::io::{stdin, Read};

#[derive(Clone, Copy, Debug)]
struct Planet {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;

    let mut planets = Vec::with_capacity(n);
    for _ in 0..n {
        let x = input.next().unwrap();
        let y = input.next().unwrap();
        let z = input.next().unwrap();
        planets.push(Planet { x, y, z });
    }

    let result = solve(n, &planets);

    println!("{result}");
}

#[derive(Clone, Copy)]
struct Edge {
    cost: u32,
    u: usize,
    v: usize,
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl DSU {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), size: vec![1; n] }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);

        if x == y {
            return false;
        }

        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }

        self.parent[y] = x;
        self.size[x] += self.size[y];
        true
    }
}

fn solve(n: usize, planets: &Vec<Planet>) -> u64 { 
    if n == 1 {
        return 0;
    }
    
    let mut idx_x: Vec<usize> = (0..n).collect();
    let mut idx_y: Vec<usize> = (0..n).collect();
    let mut idx_z: Vec<usize> = (0..n).collect();

    idx_x.sort_unstable_by_key(|&i| planets[i].x);
    idx_y.sort_unstable_by_key(|&i| planets[i].y);
    idx_z.sort_unstable_by_key(|&i| planets[i].z);

    let mut edges = Vec::with_capacity((n-1) * 3);

    let mut calc = |idx: Vec<usize>, coord: &dyn Fn(usize) -> i32| {
        for i in 0..n-1 {
            edges.push(Edge {
                cost: (coord(idx[i+1]) - coord(idx[i])) as u32,
                u: idx[i],
                v: idx[i+1],
            });
        }
    };

    calc(idx_x, &|i| planets[i].x);
    calc(idx_y, &|i| planets[i].y);
    calc(idx_z, &|i| planets[i].z);

    edges.sort_unstable_by_key(|e| e.cost);

    let mut dsu = DSU::new(n);
    let mut used: usize = 0;
    let mut ans: u64 = 0;

    for e in edges {
        if dsu.unite(e.u, e.v) {
            used += 1;
            ans += e.cost as u64;
        }

        if used == n-1 {
            break;
        }
    }
    
    ans
}