use proconio::{fastout, input};
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        m: usize,
        uv: [(usize, usize); m],
        p: [usize; 8],
    }
    let mut graph = vec![Vec::new(); 9];
    for &(u, v) in &uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    let mut pieces = [8; 9];
    for (i, &p) in p.iter().enumerate() {
        pieces[p - 1] = i;
    }
    let mut hs = HashSet::new();
    hs.insert(pieces);
    let mut vd = VecDeque::new();
    vd.push_back((pieces, 0, pieces.iter().position(|&p| p == 8).unwrap()));
    while let Some((p, min, i)) = vd.pop_front() {
        if p == [0, 1, 2, 3, 4, 5, 6, 7, 8] {
            println!("{}", min);
            return;
        }
        for &j in &graph[i] {
            let mut pieces = p;
            pieces.swap(i, j);
            if !hs.contains(&pieces) {
                hs.insert(pieces);
                vd.push_back((pieces, min + 1, j));
            }
        }
    }
    println!("-1");
}
