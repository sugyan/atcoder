use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: [String; n],
    }
    let mut hs = HashSet::with_capacity(n);
    let mut prev = None;
    let mut ok = || {
        for w in &w {
            if hs.contains(w) {
                return false;
            }
            if let Some(p) = prev {
                if !w.starts_with(p) {
                    return false;
                }
            }
            hs.insert(w);
            prev = w.chars().last();
        }
        true
    };
    println!("{}", if ok() { "Yes" } else { "No" });
}
