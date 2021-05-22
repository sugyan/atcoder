use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
    }
    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for i in 100..=x {
        dp[i] = (100..=105).any(|j| dp[i - j]);
    }
    println!("{}", if dp[x] { 1 } else { 0 });
}
