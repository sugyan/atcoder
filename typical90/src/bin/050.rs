use proconio::{fastout, input};

const MOD: i32 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize, l: usize,
    }
    let mut dp = vec![1; n + 1];
    for i in l..=n {
        dp[i] = (dp[i - 1] + dp[i - l]) % MOD;
    }
    println!("{}", dp[n]);
}
