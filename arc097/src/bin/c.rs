use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let mut hs = HashSet::new();
    for i in 1..=k {
        for s in s.windows(i) {
            hs.insert(s.iter().collect::<String>());
        }
    }
    let mut v = hs.iter().collect::<Vec<_>>();
    v.sort_unstable();
    println!("{}", v[k - 1]);
}
