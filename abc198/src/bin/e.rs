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
    dfs(&graph, 0, None, &c, &mut cs, &mut answers);
    for (i, &b) in answers.iter().enumerate() {
        if b {
            println!("{}", i + 1);
        }
    }
}

fn dfs(
    graph: &[Vec<usize>],
    i: usize,
    prev: Option<usize>,
    c: &[usize],
    cs: &mut Vec<usize>,
    answers: &mut Vec<bool>,
) {
    if cs[c[i] - 1] > 0 {
        answers[i] = false;
    }
    cs[c[i] - 1] += 1;
    for &j in &graph[i] {
        if prev != Some(j) {
            dfs(graph, j, Some(i), c, cs, answers);
        }
    }
    cs[c[i] - 1] -= 1;
}
