use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: u8, q: usize,
        abcd: [(usize, usize, u8, u32); q],
    }
    let mut answer = 0;
    for v in (1..=m).combinations_with_replacement(n) {
        let score = abcd
            .iter()
            .map(|&(a, b, c, d)| if v[b - 1] - v[a - 1] == c { d } else { 0 })
            .sum();
        answer = answer.max(score);
    }
    println!("{:?}", answer);
}
