use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        uv: [(usize, usize); m],
    }
    let mut graph = HashMap::new();
    for &(u, v) in &uv {
        graph.entry(u - 1).or_insert_with(Vec::new).push(v - 1);
        graph.entry(v - 1).or_insert_with(Vec::new).push(u - 1);
    }
    let mut visited = vec![false; n];
    let mut answer = 0;
    for i in 0..n {
        if dfs(&graph, &mut visited, i, None) {
            answer += 1;
        }
    }
    println!("{:?}", answer);
}

fn dfs(
    graph: &HashMap<usize, Vec<usize>>,
    visited: &mut Vec<bool>,
    i: usize,
    prev: Option<usize>,
) -> bool {
    if visited[i] {
        return false;
    }
    visited[i] = true;
    if let Some(v) = graph.get(&i) {
        for &j in v {
            if Some(j) != prev && !dfs(graph, visited, j, Some(i)) {
                return false;
            }
        }
    }
    true
}
