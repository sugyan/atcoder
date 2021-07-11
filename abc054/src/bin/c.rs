use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    }
    let mut graph = vec![vec![false; n]; n];
    for &(a, b) in &ab {
        graph[a - 1][b - 1] = true;
        graph[b - 1][a - 1] = true;
    }
    let mut answer = 0;
    for mut p in (1..n).permutations(n - 1) {
        p.push(0);
        if p.windows(2).all(|e| graph[e[0]][e[1]]) {
            answer += 1;
        }
    }
    println!("{}", answer);
}
