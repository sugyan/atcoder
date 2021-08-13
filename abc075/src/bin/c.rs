use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    }
    let mut graph = vec![Vec::new(); n];
    for &(a, b) in &ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut answer = 0;
    for e in &ab {
        let mut visited = vec![false; n];
        dfs(&graph, 0, e, &mut visited);
        if !visited.iter().all(|&b| b) {
            answer += 1;
        }
    }
    println!("{}", answer);
}

fn dfs(graph: &[Vec<usize>], i: usize, e: &(usize, usize), visited: &mut Vec<bool>) {
    visited[i] = true;
    for &j in &graph[i] {
        if e == &(i + 1, j + 1) || e == &(j + 1, i + 1) {
            continue;
        }
        if !visited[j] {
            dfs(graph, j, e, visited);
        }
    }
}
