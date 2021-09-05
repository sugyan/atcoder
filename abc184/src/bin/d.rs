use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize, b: usize, c: usize,
    }
    let mut dp = vec![vec![vec![0.0; 101]; 101]; 101];
    for i in (0..300).rev() {
        for j in (0..=99.min(i)).rev() {
            for k in (0..=99.min(i - j)).rev() {
                let l = i - j - k;
                if l > 99 {
                    continue;
                }
                dp[j][k][l] += (1.0 + dp[j + 1][k][l]) * j as f64 / i as f64;
                dp[j][k][l] += (1.0 + dp[j][k + 1][l]) * k as f64 / i as f64;
                dp[j][k][l] += (1.0 + dp[j][k][l + 1]) * l as f64 / i as f64;
                if j == a && k == b && l == c {
                    println!("{:.10}", dp[j][k][l]);
                    return;
                }
            }
        }
    }
}
