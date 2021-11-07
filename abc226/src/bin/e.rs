use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        uv: [(usize, usize); m],
    }
    let mut uf = UnionFind::new(n);
    let mut graph = vec![Vec::new(); n];
    for &(u, v) in &uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
        uf.union(u - 1, v - 1);
    }
    let mut edges = vec![Vec::new(); n];
    for i in 0..n {
        edges[uf.find(i)].push(graph[i].len());
    }
    let mut answer = 1;
    for v in &edges {
        if v.is_empty() {
            continue;
        }
        answer = if v.iter().sum::<usize>() == v.len() * 2 {
            answer * 2 % MOD
        } else {
            0
        };
    }
    println!("{}", answer);
}
