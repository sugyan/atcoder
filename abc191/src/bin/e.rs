use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        abc: [(usize, usize, u32); m],
    }
    let mut graph = vec![Vec::new(); n];
    for &(a, b, c) in &abc {
        graph[a - 1].push((b - 1, c));
    }
    let mut mins = vec![vec![None; n]; n];
    for (i, m) in mins.iter_mut().enumerate() {
        dijkstra(&graph, i, m);
    }
    for i in 0..n {
        let mut min = None;
        for j in 0..n {
            if i != j {
                if let (Some(m0), Some(m1)) = (mins[i][j], mins[j][i]) {
                    min = Some(min.map_or(m0 + m1, |m: u32| m.min(m0 + m1)));
                }
            }
        }
        for &(j, c) in &graph[i] {
            if j == i {
                min = Some(min.map_or(c, |m: u32| m.min(c)));
            }
        }
        println!("{}", min.map_or(-1, |m| m as i32));
    }
}

fn dijkstra(graph: &[Vec<(usize, u32)>], i: usize, mins: &mut Vec<Option<u32>>) {
    let mut bh = BinaryHeap::new();
    bh.push((Reverse(0), i));
    while let Some((Reverse(min), i)) = bh.pop() {
        if mins[i].is_some() {
            continue;
        }
        mins[i] = Some(min);
        for &(j, c) in &graph[i] {
            if mins[j].is_none() {
                bh.push((Reverse(min + c), j));
            }
        }
    }
}
