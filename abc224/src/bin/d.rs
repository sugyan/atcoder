use proconio::{fastout, input};
use std::collections::{HashMap, VecDeque};

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
    let mut pieces = vec![8; 9];
    for (i, &p) in p.iter().enumerate() {
        pieces[p - 1] = i;
    }
    let mut hm = HashMap::new();
    let mut vd = VecDeque::new();
    vd.push_back((pieces, 0, None));
    while let Some((p, min, prev)) = vd.pop_front() {
        if hm.contains_key(&p) {
            continue;
        }
        hm.insert(p.clone(), min);
        if let Some(i) = p.iter().position(|&p| p == 8) {
            for &j in &graph[i] {
                if prev != Some(j) {
                    let mut pieces = p.clone();
                    pieces.swap(i, j);
                    vd.push_back((pieces, min + 1, Some(i)));
                }
            }
        }
    }
    println!("{}", hm.get(&(0..9).collect::<Vec<_>>()).unwrap_or(&-1));
}
