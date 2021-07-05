use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, d: usize,
        a: [[usize; w]; h],
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut hm = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            hm.insert(a[i][j], (i as isize, j as isize));
        }
    }
    let mut v = vec![0; h * w];
    for i in 0..h * w - d {
        if let (Some(p), Some(q)) = (hm.get(&(i + 1)), hm.get(&(i + 1 + d))) {
            v[i + d] = v[i] + (p.0 - q.0).abs() + (p.1 - q.1).abs();
        }
    }
    // println!("{:?}", v);
    for &(l, r) in &lr {
        println!("{}", v[r - 1] - v[l - 1]);
    }
}
