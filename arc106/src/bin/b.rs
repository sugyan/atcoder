use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n],
        b: [i64; n],
        cd: [(usize, usize); m],
    }
    let mut uf = UnionFind::new(n);
    for &(c, d) in &cd {
        uf.union(c - 1, d - 1);
    }
    let mut suma = vec![0; n];
    let mut sumb = vec![0; n];
    for i in 0..n {
        suma[uf.find(i)] += a[i];
        sumb[uf.find(i)] += b[i];
    }
    println!("{}", if suma == sumb { "Yes" } else { "No" });
}
