use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let yes = a.iter().collect::<HashSet<_>>().len() == n;
    println!("{}", if yes { "YES" } else { "NO" });
}
