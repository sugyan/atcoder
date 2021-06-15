use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, k: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); k],
    }
    let mut v = vec![-1; n];
    let mut uf = UnionFind::new(n);
    for &(a, b) in &ab {
        v[a - 1] -= 1;
        v[b - 1] -= 1;
        uf.union(a - 1, b - 1);
    }
    for &(c, d) in &cd {
        if uf.equiv(c - 1, d - 1) {
            v[c - 1] -= 1;
            v[d - 1] -= 1;
        }
    }
    let counts = (0..n).fold(vec![0; n], |mut acc, x| {
        acc[uf.find(x)] += 1;
        acc
    });
    let answers = v
        .iter()
        .enumerate()
        .map(|(i, &e)| (e + counts[uf.find(i)]).to_string())
        .collect::<Vec<_>>();
    println!("{}", answers.join(" "));
}
