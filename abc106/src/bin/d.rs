use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, q: usize,
        lr: [(usize, usize); m],
        pq: [(usize, usize); q],
    }
    let mut g = vec![vec![0; n + 1]; n + 1];
    lr.iter().for_each(|&(l, r)| g[l][r] += 1);
    for v in g.iter_mut() {
        (1..=n).for_each(|j| v[j] += v[j - 1]);
    }
    for &(p, q) in &pq {
        println!("{}", g[p..=q].iter().map(|v| v[q] - v[p - 1]).sum::<u32>());
    }
}
