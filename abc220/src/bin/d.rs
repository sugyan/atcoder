use proconio::{fastout, input};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut dp = vec![vec![0; 10]; n];
    dp[0][a[0] as usize] += 1;
    for i in 1..n {
        for j in 0..=9 {
            dp[i][(j as u64 + a[i]) as usize % 10] += dp[i - 1][j];
            dp[i][(j as u64 * a[i]) as usize % 10] += dp[i - 1][j];
        }
        dp[i].iter_mut().for_each(|val| *val %= MOD);
    }
    for answer in &dp[n - 1] {
        println!("{}", answer);
    }
}
