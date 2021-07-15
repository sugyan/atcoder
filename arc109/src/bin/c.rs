use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize, k: usize,
        mut s: Chars,
    }
    for _ in 0..k {
        s = s
            .repeat(2)
            .chunks(2)
            .map(|c| match (c[0], c[1]) {
                ('R', 'P') | ('P', 'S') | ('S', 'R') => c[1],
                _ => c[0],
            })
            .collect();
    }
    println!("{}", s[0]);
}
