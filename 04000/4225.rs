use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut cnt = 1;

    loop {
        let n = match lines.next() {
            Some(Ok(line)) => line.trim().parse::<usize>().unwrap(),
            _ => break,
        };

        if n == 0 {
            break;
        }

        let mut res: Option<f64> = None;

        let mut polygon = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(Ok(line)) = lines.next() {
                let coords: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                polygon.push((coords[0], coords[1]));
            }
        }
        
        polygon = convex_hull(polygon);
        let len = polygon.len();

        for i in 0..len {
            let (a, b, c) = line_from_points(polygon[i], polygon[(i+1) % len]);
            let mut dist_max: Option<f64> = None;
            for j in 2..len {
                let s = on_same_side(a, b, c, polygon[(i+j) % len]);
                let dist = point_line_distance(a, b, s);
                match dist_max {
                    None => { dist_max = Some(dist); }
                    Some(ans) => { if ans < dist { dist_max = Some(dist); } }
                }
            }
            match res {
                None => { res = Some(ceil_to_cent(dist_max.unwrap())); }
                Some(ans) => { if ans > dist_max.unwrap() { res = Some(ceil_to_cent(dist_max.unwrap())); } }
            }
        }


        println!("Case {}: {:.2}", cnt, res.unwrap());
        cnt += 1;
    }
}

fn line_from_points(p1: (i32, i32), p2: (i32, i32)) -> (i32, i32, i32) {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let a = y1 - y2;
    let b = x2 - x1;
    let c = x1 * y2 - x2 * y1;

    (a, b, c)
}

fn on_same_side(a: i32, b: i32, c: i32, points: (i32, i32)) -> i32 {
    a * points.0 + b * points.1 + c
}

fn point_line_distance(a: i32, b: i32, s: i32) -> f64 {
    let numerator = s.abs() as f64;
    let denominator = ((a * a + b * b) as f64).sqrt();
    numerator / denominator
}

fn ceil_to_cent(value: f64) -> f64 {
    let eps = 1e-9;
    ((value * 100.0 - eps).ceil()) / 100.0
}

fn cross(o: (i32, i32), a: (i32, i32), b: (i32, i32)) -> i64 {
    (a.0 as i64 - o.0 as i64) * (b.1 as i64 - o.1 as i64)
        - (a.1 as i64 - o.1 as i64) * (b.0 as i64 - o.0 as i64)
}

fn convex_hull(mut pts: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if pts.len() <= 2 {
        return pts;
    }
    pts.sort_by(|a, b| if a.0 == b.0 { a.1.cmp(&b.1) } else { a.0.cmp(&b.0) });

    let mut lower = Vec::new();
    for &p in &pts {
        while lower.len() >= 2
            && cross(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0
        {
            lower.pop();
        }
        lower.push(p);
    }

    let mut upper = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2
            && cross(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 0
        {
            upper.pop();
        }
        upper.push(p);
    }

    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}