use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    }
    let mut uf = UnionFind::new(n);
    for &(a, b) in &ab {
        uf.union(a - 1, b - 1);
    }
    let mut v = vec![false; n];
    for i in 0..n {
        v[uf.find(i)] = true;
    }
    let answer = v.iter().filter(|&b| *b).count() - 1;
    println!("{}", answer);
}
