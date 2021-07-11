use itertools::Itertools;
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
    for mut p in (1..n).permutations(n - 1) {
        p.push(0);
        if p.windows(2).all(|e| graph[e[0]].contains(&e[1])) {
            answer += 1;
        }
    }
    println!("{}", answer);
}
