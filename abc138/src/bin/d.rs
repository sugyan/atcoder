use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        ab: [(usize, usize); n - 1],
        pq: [(usize, u32); q],
    }
    let mut graph = vec![Vec::new(); n];
    for &(a, b) in &ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut values = vec![0; n];
    for &(p, q) in &pq {
        values[p - 1] += q;
    }
    let mut vd = VecDeque::new();
    vd.push_back((0, None));
    while let Some((i, prev)) = vd.pop_front() {
        for &j in &graph[i] {
            if prev != Some(j) {
                values[j] += values[i];
                vd.push_back((j, Some(i)));
            }
        }
    }
    let answers = values.iter().map(|v| v.to_string()).collect::<Vec<_>>();
    println!("{}", answers.join(" "));
}
