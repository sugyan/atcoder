use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: String,
        k: usize,
    }
    let n = n.bytes().map(|u| (u - b'0') as u32).collect::<Vec<_>>();
    let mut dp = vec![vec![vec![0; 2]; k + 2]; n.len() + 1];
    dp[0][0][0] = 1;
    for i in 0..n.len() {
        for j in 0..=k {
            for d in 0..10 {
                let jj = j + if d != 0 { 1 } else { 0 };
                for l in 0..2 {
                    if n[i] < d && l == 0 {
                        continue;
                    }
                    dp[i + 1][jj][if d < n[i] { 1 } else { l }] += dp[i][j][l];
                }
            }
        }
    }
    println!("{}", dp[n.len()][k][0] + dp[n.len()][k][1]);
}
