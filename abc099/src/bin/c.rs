use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut dp = vec![n; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        dp[i] = dp[i].min(dp[i - 1] + 1);
        (1..)
            .map(|i| 6_usize.pow(i))
            .take_while(|&m| m <= i)
            .for_each(|m| dp[i] = dp[i].min(dp[i - m] + 1));
        (1..)
            .map(|i| 9_usize.pow(i))
            .take_while(|&m| m <= i)
            .for_each(|m| dp[i] = dp[i].min(dp[i - m] + 1));
    }
    println!("{}", dp[n]);
}
