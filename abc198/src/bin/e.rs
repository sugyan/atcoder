use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: [usize; n],
        ab: [(usize, usize); n - 1],
    }
    let mut graph = vec![Vec::new(); n];
    for &(a, b) in &ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut cs = vec![0; *c.iter().max().unwrap()];
    let mut answers = vec![true; n];
    dfs(&graph, 0, n, &c, &mut cs, &mut answers);
    for (i, _) in answers.iter().enumerate().filter(|(_, &b)| b) {
        println!("{}", i + 1);
    }
}

fn dfs(g: &[Vec<usize>], i: usize, p: usize, c: &[usize], cs: &mut [usize], answers: &mut [bool]) {
    if cs[c[i] - 1] > 0 {
        answers[i] = false;
    }
    cs[c[i] - 1] += 1;
    for &j in &g[i] {
        if p != j {
            dfs(g, j, i, c, cs, answers);
        }
    }
    cs[c[i] - 1] -= 1;
}
