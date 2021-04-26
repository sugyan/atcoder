use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }
    let answer = a
        .iter_mut()
        .map(|a| {
            while *a & 1 == 0 {
                *a >>= 1;
            }
            a
        })
        .collect::<HashSet<_>>()
        .len();
    println!("{}", answer);
}
