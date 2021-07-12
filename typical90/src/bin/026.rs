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
    let mut c = vec![None; n];
    dfs(&graph, 0, None, true, &mut c);
    let target = c.iter().filter(|&o| *o == Some(true)).count() >= n / 2;
    for i in (0..n).filter(|&i| c[i] == Some(target)).take(n / 2) {
        println!("{}", i + 1);
    }
}

fn dfs(graph: &[Vec<usize>], i: usize, prev: Option<usize>, val: bool, c: &mut Vec<Option<bool>>) {
    c[i] = Some(val);
    for &j in &graph[i] {
        if prev != Some(j) {
            dfs(graph, j, Some(i), !val, c);
        }
    }
}
