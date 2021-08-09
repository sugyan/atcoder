use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};
use std::collections::HashSet;

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
    let mut groups = vec![HashSet::new(); n];
    for i in 0..n {
        groups[uf.find(i)].insert(i);
    }
    let mut answer = 0;
    for g in &groups {
        for &i in g {
            if g.contains(&(p[i] - 1)) {
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}
