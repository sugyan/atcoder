use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut hm = HashMap::new();
    a.iter().for_each(|&a| *hm.entry(a).or_insert(0) += 1);
    let answer = hm.values().filter(|&v| *v & 1 > 0).count();
    println!("{}", answer);
}
