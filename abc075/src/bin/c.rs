use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    }
    let mut answer = 0;
    for i in 0..m {
        let mut uf = UnionFind::new(n);
        for j in (0..m).filter(|&j| j != i) {
            uf.union(ab[j].0 - 1, ab[j].1 - 1);
        }
        if !(1..n).all(|j| uf.equiv(0, j)) {
            answer += 1;
        }
    }
    println!("{}", answer);
}
