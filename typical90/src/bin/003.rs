use proconio::{fastout, input};

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
    let mut depths = vec![0; n];
    dfs(&graph, 0, None, 0, &mut depths);
    let max = (0..n).max_by_key(|&i| depths[i]).unwrap();
    dfs(&graph, max, None, 0, &mut depths);
    println!("{:?}", depths.iter().max().unwrap() + 1);
}

fn dfs(graph: &[Vec<usize>], i: usize, prev: Option<usize>, d: usize, depths: &mut Vec<usize>) {
    depths[i] = d;
    for &j in &graph[i] {
        if prev != Some(j) {
            dfs(graph, j, Some(i), d + 1, depths);
        }
    }
}
