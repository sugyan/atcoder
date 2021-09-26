use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let size = 400_001;
    let mut uf = UnionFind::new(size);
    for &(a, b) in &ab {
        uf.union(a, b);
    }
    let (mut nodes, mut edges) = (vec![0; size], vec![0; size]);
    (0..size).for_each(|i| nodes[uf.find(i)] += 1);
    ab.iter().for_each(|&(a, _)| edges[uf.find(a)] += 1);
    let answer = (0..size).map(|i| nodes[i].min(edges[i])).sum::<u32>();
    println!("{}", answer);
}
