use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let mut graph = vec![Vec::new(); n];
    for &(a, b) in &ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut hm = HashMap::new();
    let i = (0..n).max_by_key(|&i| graph[i].len()).unwrap();
    let max = graph[i].len();
    dfs(&graph, &mut hm, i, None, max);
    println!("{}", max);
    for &(a, b) in &ab {
        if let Some(&answer) = hm.get(&(a - 1, b - 1)) {
            println!("{}", answer);
        }
    }
}

fn dfs(
    graph: &[Vec<usize>],
    hm: &mut HashMap<(usize, usize), usize>,
    i: usize,
    prev: Option<(usize, usize)>,
    max: usize,
) {
    let mut k = 1;
    for &j in &graph[i] {
        match prev {
            Some((p, _)) if p == j => continue,
            _ => {}
        }
        let val = (prev.map_or(0, |p| p.1) + k) % max;
        hm.insert((i.min(j), i.max(j)), val + 1);
        dfs(graph, hm, j, Some((i, val)), max);
        k += 1;
    }
}
