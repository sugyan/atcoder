use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize, b: usize, c: usize,
    }
    let mut dp = vec![vec![vec![0.0; 101]; 101]; 101];
    for i in (0..100).rev() {
        for j in (0..100).rev() {
            for k in (0..100).rev() {
                let sum = (i + j + k) as f64;
                dp[i][j][k] = 1.0
                    + dp[i + 1][j][k] * i as f64 / sum
                    + dp[i][j + 1][k] * j as f64 / sum
                    + dp[i][j][k + 1] * k as f64 / sum;
            }
        }
    }
    println!("{:.10}", dp[a][b][c]);
}
