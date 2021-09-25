use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut dp = vec![vec![0; 13]; s.len() + 1];
    dp[0][0] = 1;
    for (i, b) in s.bytes().enumerate() {
        for j in 0..13 {
            for k in 0..10 {
                if b == b'?' || b - b'0' == k {
                    dp[i + 1][(j * 10 + k as usize) % 13] += dp[i][j];
                }
            }
        }
        (0..13).for_each(|j| dp[i + 1][j] %= MOD);
    }
    println!("{}", dp[s.len()][5]);
}
