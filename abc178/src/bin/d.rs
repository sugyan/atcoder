use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        s: usize,
    }
    let mut dp = vec![0; s + 1];
    dp[0] = 1;
    (3..=s).for_each(|i| dp[i] = (dp[i - 1] + dp[i - 3]) % MOD);
    println!("{}", dp[s]);
}
