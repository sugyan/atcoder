use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        p: [usize; n],
        xy: [(usize, usize); m],
    }
    let mut uf = UnionFind::new(n);
    for &(x, y) in &xy {
        uf.union(x - 1, y - 1);
    }
    let answer = (0..n).filter(|&i| uf.equiv(i, p[i] - 1)).count();
    println!("{}", answer);
}
