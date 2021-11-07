use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: [[u32]; n],
    }
    println!("{}", l.iter().collect::<HashSet<_>>().len());
}
