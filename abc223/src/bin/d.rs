use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    }
    let mut g = vec![(0, Vec::new()); n];
    for &(a, b) in &ab {
        g[b - 1].0 += 1;
        g[a - 1].1.push(b - 1);
    }
    let mut bh = BinaryHeap::new();
    for i in (0..n).filter(|&i| g[i].0 == 0) {
        bh.push(Reverse(i));
    }
    let mut v = Vec::with_capacity(n);
    while let Some(Reverse(i)) = bh.pop() {
        v.push((i + 1).to_string());
        for &j in &g[i].1.clone() {
            g[j].0 -= 1;
            if g[j].0 == 0 {
                bh.push(Reverse(j));
            }
        }
    }
    if v.len() == n {
        println!("{}", v.join(" "));
    } else {
        println!("-1");
    };
}
