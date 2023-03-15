use proconio::marker::Bytes;
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: [Bytes; n],
    }
    let ok = w.iter().collect::<HashSet<_>>().len() == n
        && w.windows(2).all(|w| w[0].iter().last() == w[1].first());
    println!("{}", if ok { "Yes" } else { "No" });
}
