use proconio::{fastout, input};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        k: usize, n: i64, m: i64,
        a: [i64; k],
    }
    let mut b = a.iter().map(|a| a * m / n).collect::<Vec<_>>();
    let mut sum = b.iter().sum::<i64>();
    let mut bh = (0..k)
        .map(|i| (a[i] * m - b[i] * n, i))
        .collect::<BinaryHeap<_>>();
    while m - sum > 0 {
        if let Some((_, i)) = bh.pop() {
            b[i] += 1;
            bh.push((a[i] * m - b[i] * n, i));
        }
        sum += 1;
    }
    println!(
        "{}",
        b.iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
