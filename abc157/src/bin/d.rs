use proconio::{fastout, input};
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, k: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); k],
    }
    let mut graph = vec![Vec::new(); n];
    let mut block = vec![HashSet::new(); n];
    for &(a, b) in &ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    for &(c, d) in &cd {
        block[c - 1].insert(d - 1);
        block[d - 1].insert(c - 1);
    }
    let mut seen = vec![false; n];
    let mut answers = vec![0; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }
        let mut hs = HashSet::new();
        let mut vd = VecDeque::new();
        vd.push_back(i);
        while let Some(i) = vd.pop_front() {
            hs.insert(i);
            for &j in &graph[i] {
                if !seen[j] {
                    seen[j] = true;
                    vd.push_back(j);
                }
            }
        }
        for &j in &hs {
            answers[j] = hs.len() - graph[j].len() - 1 - block[j].intersection(&hs).count();
        }
    }
    println!(
        "{}",
        answers
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
