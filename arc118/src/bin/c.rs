use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![6];
    v.extend((18..10000).step_by(6));
    v.extend((10..10000).step_by(10));
    v.extend((15..10000).step_by(15));
    v.sort_unstable();
    v.dedup();
    println!("{}", v.iter().take(n).join(" "));
}
