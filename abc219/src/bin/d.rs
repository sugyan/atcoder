use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize, y: usize,
        ab: [(usize, usize); n],
    }
    let mut dp = vec![vec![vec![n + 1; 301]; 301]; n + 1];
    dp[0][0][0] = 0;
    for (i, &(a, b)) in ab.iter().enumerate() {
        for j in 0..=x {
            for k in 0..=y {
                let ja = x.min(j + a);
                let kb = y.min(k + b);
                dp[i + 1][ja][kb] = dp[i + 1][ja][kb].min(dp[i][j][k] + 1);
                dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k]);
            }
        }
    }
    if dp[n][x][y] > n {
        println!("-1");
    } else {
        println!("{}", dp[n][x][y]);
    };
}
