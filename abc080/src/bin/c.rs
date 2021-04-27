use std::usize;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        f: [[u8; 10]; n],
        p: [[i64; 11]; n],
    }
    let f = f
        .iter()
        .map(|v| {
            v.iter()
                .enumerate()
                .map(|(i, &u)| if u == 1 { 1 << i } else { 0 })
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    let mut answer = std::i64::MIN;
    for i in 1..1 << 10 {
        let sum = (0..n)
            .map(|j| p[j][(f[j] & i).count_ones() as usize])
            .sum::<i64>();
        answer = answer.max(sum);
    }
    println!("{}", answer);
}
