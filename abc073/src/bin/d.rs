use itertools::Itertools;
use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize, mr: (usize, usize),
        r: [usize; mr.1],
        abc: [(usize, usize, u32); mr.0],
    }
    let mut graph = vec![Vec::new(); n];
    for &(a, b, c) in &abc {
        graph[a - 1].push((b - 1, c));
        graph[b - 1].push((a - 1, c));
    }
    let mut mins = vec![vec![None; n]; n];
    for &r in &r {
        mins[r - 1][r - 1] = Some(0);
        let mut bh = BinaryHeap::new();
        bh.push((Reverse(0), r - 1));
        while let Some((Reverse(min), i)) = bh.pop() {
            mins[r - 1][i] = Some(mins[r - 1][i].map_or(min, |m| m.min(min)));
            for &(j, c) in &graph[i] {
                if mins[r - 1][j].is_none() {
                    bh.push((Reverse(min + c), j));
                }
            }
        }
    }
    let answer = r
        .iter()
        .permutations(mr.1)
        .map(|p| {
            p.windows(2)
                .filter_map(|w| mins[w[0] - 1][w[1] - 1])
                .sum::<u32>()
        })
        .min()
        .unwrap();
    println!("{}", answer);
}
