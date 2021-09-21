use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        abc: [(usize, usize, i32); m],
    }
    let mut graph = vec![Vec::new(); n];
    for &(a, b, c) in &abc {
        graph[a - 1].push((b - 1, c));
    }
    for i in 0..n {
        let mut mins = vec![None; graph.len()];
        let mut bh = BinaryHeap::new();
        bh.push((Reverse(0), i));
        while let Some((Reverse(min), i)) = bh.pop() {
            for &(j, c) in &graph[i] {
                if let Some(m) = mins[j] {
                    if min + c > m {
                        continue;
                    }
                }
                mins[j] = Some(min + c);
                bh.push((Reverse(min + c), j));
            }
        }
        println!("{}", mins[i].unwrap_or(-1));
    }
}
