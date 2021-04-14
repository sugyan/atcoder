use proconio::{fastout, input};
use std::iter::successors;

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        dp[i] = dp[i - 1] + 1;
        successors(Some(1), |&j| Some(j * 6).filter(|&j| j <= i))
            .for_each(|j| dp[i] = dp[i].min(dp[i - j] + 1));
        successors(Some(1), |&j| Some(j * 9).filter(|&j| j <= i))
            .for_each(|j| dp[i] = dp[i].min(dp[i - j] + 1));
    }
    println!("{}", dp[n]);
}
