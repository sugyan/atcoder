use proconio::{fastout, input};

const DIV: i64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        lr: [(usize, usize); k],
    }
    let mut dp = vec![0_i64; n];
    dp[0] = 1;
    dp[1] = DIV - 1;
    for i in 0..n - 1 {
        for &(l, r) in &lr {
            if i + l < n {
                dp[i + l] = (dp[i + l] + dp[i]) % DIV;
            }
            if i + r + 1 < n {
                dp[i + r + 1] = (dp[i + r + 1] + DIV - dp[i]) % DIV;
            }
        }
        dp[i + 1] = (dp[i + 1] + dp[i]) % DIV;
    }
    println!("{}", dp[n - 1]);
}
