use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        a: [Chars; h],
    }
    let f = |i: usize, j: usize| if a[i][j] == '+' { 1 } else { -1 };
    let mut dp = vec![vec![0; w]; h];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            dp[i][j] = match (i + 1 < h, j + 1 < w) {
                (true, true) => (f(i + 1, j) - dp[i + 1][j]).max(f(i, j + 1) - dp[i][j + 1]),
                (true, false) => f(i + 1, j) - dp[i + 1][j],
                (false, true) => f(i, j + 1) - dp[i][j + 1],
                _ => 0,
            }
        }
    }
    let answer = match dp[0][0].cmp(&0) {
        Ordering::Less => "Aoki",
        Ordering::Equal => "Draw",
        Ordering::Greater => "Takahashi",
    };
    println!("{}", answer);
}
