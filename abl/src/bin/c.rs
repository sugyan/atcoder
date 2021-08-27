use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    }
    let mut answer = n - 1;
    let mut uf = UnionFind::new(n);
    for &(a, b) in &ab {
        if uf.union(a - 1, b - 1) {
            answer -= 1;
        }
    }
    println!("{}", answer);
}
