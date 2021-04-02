use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        na: usize, nb: usize,
        a: [u32; na],
        b: [u32; nb],
    }
    let a = a.iter().collect::<HashSet<_>>();
    let b = b.iter().collect::<HashSet<_>>();
    println!(
        "{:.10}",
        (f64::from(a.intersection(&b).count() as i32) / f64::from(a.union(&b).count() as i32))
    )
}
