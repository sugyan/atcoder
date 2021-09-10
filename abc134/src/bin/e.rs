use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut v = Vec::new();
    for &a in &a {
        if let Err(i) = v.binary_search_by(|&probe| {
            if probe < a {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }) {
            if i == 0 {
                v.insert(0, a)
            } else {
                v[i - 1] = a
            }
        };
    }
    println!("{}", v.len());
}
