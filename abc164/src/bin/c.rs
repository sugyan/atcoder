use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    println!("{}", s.iter().collect::<HashSet<_>>().len());
}
