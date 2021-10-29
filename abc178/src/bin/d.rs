use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        s: usize,
    }
    let mut dp = vec![0; s + 1];
    for i in 3..=s {
        dp[i] = 1 + (3..=i - 3).map(|j| dp[j]).sum::<u64>() % MOD;
    }
    println!("{}", dp[s]);
}
