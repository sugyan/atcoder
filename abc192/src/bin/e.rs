use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, x: usize, y: usize,
        abtk: [(usize, usize, u64, u64); m],
    }
    let mut graph = vec![Vec::new(); n];
    for &(a, b, t, k) in &abtk {
        graph[a - 1].push((b - 1, t, k));
        graph[b - 1].push((a - 1, t, k));
    }
    let mut dists = vec![None; n];
    let mut bh = BinaryHeap::new();
    bh.push((Reverse(0), x - 1));
    while let Some((Reverse(min), src)) = bh.pop() {
        if dists[src].is_some() {
            continue;
        }
        dists[src] = Some(min);
        for &(dst, t, k) in &graph[src] {
            if dists[dst].is_none() {
                let wait = if min % k == 0 { 0 } else { k - min % k };
                bh.push((Reverse(min + wait + t), dst));
            }
        }
    }
    let answer = if let Some(d) = dists[y - 1] {
        d as i64
    } else {
        -1
    };
    println!("{}", answer);
}
