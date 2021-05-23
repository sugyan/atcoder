use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, u64); n - 1],
        q: usize, k: usize,
        xy: [(usize, usize); q],
    }
    let mut graph = vec![Vec::new(); n];
    for &(a, b, c) in &abc {
        graph[a - 1].push((b - 1, c));
        graph[b - 1].push((a - 1, c));
    }
    let mut dist = vec![0; n];
    dfs(&graph, k - 1, 0, None, &mut dist);
    for &(x, y) in &xy {
        println!("{}", dist[x - 1] + dist[y - 1]);
    }
}

fn dfs(graph: &[Vec<(usize, u64)>], i: usize, d: u64, prev: Option<usize>, dist: &mut Vec<u64>) {
    dist[i] = d;
    for &(j, c) in &graph[i] {
        if prev != Some(j) {
            dfs(graph, j, d + c, Some(i), dist);
        }
    }
}
