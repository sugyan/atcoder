use proconio::{fastout, input};

fn dfs(graph: &[Vec<(usize, usize)>], a: &mut Vec<usize>, i: usize, p: Option<(usize, usize)>) {
    let mut c = 1;
    for &(j, k) in &graph[i] {
        match p {
            Some((pi, _)) if pi == j => continue,
            Some((_, pc)) if pc == c => c += 1,
            _ => {}
        }
        a[k] = c;
        dfs(graph, a, j, Some((i, c)));
        c += 1;
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let mut graph = vec![Vec::new(); n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        graph[a - 1].push((b - 1, i));
        graph[b - 1].push((a - 1, i));
    }
    let mut a = vec![0; n - 1];
    dfs(&&graph, &mut a, 0, None);
    println!("{}", a.iter().max().unwrap());
    for &a in &a {
        println!("{}", a);
    }
}
