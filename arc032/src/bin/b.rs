use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m]
    }
    let mut g = vec![Vec::new(); n];
    for &(a, b) in &ab {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let mut answer = -1;
    let mut v = vec![false; n];
    for i in 0..n {
        if !v[i] {
            answer += 1;
            let mut vd = VecDeque::new();
            vd.push_back(i);
            while let Some(f) = vd.pop_front() {
                v[f] = true;
                for &d in &g[f] {
                    if !v[d] {
                        vd.push_back(d);
                    }
                }
            }
        }
    }
    println!("{:?}", answer);
}
