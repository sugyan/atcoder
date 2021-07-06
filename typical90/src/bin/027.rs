use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut hs = HashSet::new();
    for (i, s) in s.iter().enumerate() {
        if !hs.contains(s) {
            println!("{}", i + 1);
        }
        hs.insert(s);
    }
}
