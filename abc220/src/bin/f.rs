use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n - 1],
    }
    let mut graph = vec![Vec::new(); n];
    for &(u, v) in &uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    let (mut answers, mut c) = (vec![0; n], vec![1; n]);
    dfs(&graph, 0, None, &mut answers, &mut c);
    let mut stack = vec![(0, None)];
    while let Some((i, prev)) = stack.pop() {
        for &j in graph[i].iter().filter(|&j| Some(*j) != prev) {
            answers[j] = answers[i] + n as u64 - c[j] * 2;
            stack.push((j, Some(i)));
        }
    }
    for answer in &answers {
        println!("{}", answer);
    }
}

fn dfs(graph: &[Vec<usize>], i: usize, p: Option<usize>, a: &mut Vec<u64>, c: &mut Vec<u64>) {
    for &j in graph[i].iter().filter(|&j| Some(*j) != p) {
        dfs(graph, j, Some(i), a, c);
        c[i] += c[j];
        a[i] += c[j] + a[j];
    }
}
