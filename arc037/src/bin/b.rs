use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        uv: [(usize, usize); m],
    }
    let mut graph = vec![Vec::new(); n];
    for &(u, v) in &uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    let mut visited = vec![false; n];
    println!(
        "{}",
        (0..n)
            .filter(|&i| dfs(&graph, &mut visited, i, None))
            .count()
    );
}

fn dfs(graph: &[Vec<usize>], visited: &mut Vec<bool>, i: usize, prev: Option<usize>) -> bool {
    if visited[i] {
        return false;
    }
    visited[i] = true;
    for &j in &graph[i] {
        if Some(j) != prev && !dfs(graph, visited, j, Some(i)) {
            return false;
        }
    }
    true
}
