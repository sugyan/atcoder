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
    let mut stack = Vec::new();
    stack.push((0, None, true));
    while let Some((i, p, v)) = stack.pop() {
        c[i] = Some(v);
        for &j in &graph[i] {
            if p != Some(j) {
                stack.push((j, Some(i), !v));
            }
        }
    }
    let target = c.iter().filter(|&o| *o == Some(true)).count() >= n / 2;
    for i in (0..n).filter(|&i| c[i] == Some(target)).take(n / 2) {
        println!("{}", i + 1);
    }
}
