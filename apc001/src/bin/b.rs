use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }
    let mut sum = 0;
    for i in 0..n {
        match a[i].cmp(&b[i]) {
            Ordering::Less => sum -= (b[i] - a[i]) / 2,
            Ordering::Equal => {}
            Ordering::Greater => sum += a[i] - b[i],
        }
    }
    println!("{}", if sum > 0 { "No" } else { "Yes" });
}
