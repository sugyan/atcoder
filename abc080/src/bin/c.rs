use std::usize;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        f: [[u8; 10]; n],
        p: [[i64; 11]; n],
    }
    let mut answer = std::i64::MIN;
    for i in 1..1 << 10 {
        let sum = (0..n)
            .map(|j| {
                p[j][(0..10)
                    .filter(|&k| f[j][k] > 0 && ((i >> k) & 1 > 0))
                    .count()]
            })
            .sum::<i64>();
        answer = answer.max(sum);
    }
    println!("{}", answer);
}
