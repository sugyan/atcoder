use num_integer::gcd;
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut hs = HashSet::new();
    for i in 0..n {
        for j in (0..n).filter(|&j| j != i) {
            let x = xy[i].0 - xy[j].0;
            let y = xy[i].1 - xy[j].1;
            let g = gcd(x.abs(), y.abs());
            hs.insert((x / g, y / g));
        }
    }
    println!("{}", hs.len());
}
