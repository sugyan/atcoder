use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut uvw: [(usize, usize, u64); n - 1],
    }
    uvw.sort_unstable_by_key(|&(_, _, w)| w);
    let mut c = vec![1; n];
    let mut uf = UnionFind::new(n);
    let mut answer = 0;
    for &(u, v, w) in &uvw {
        let cu = c[uf.find(u - 1)];
        let cv = c[uf.find(v - 1)];
        answer += cu * cv * w;
        uf.union(u - 1, v - 1);
        c[uf.find(u - 1)] = cu + cv;
    }
    println!("{}", answer);
}
