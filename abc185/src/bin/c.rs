use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: usize,
    }
    let mut dp = vec![vec![0_u64; 13]; l + 1];
    dp[0][0] = 1;
    for i in 0..=l {
        for j in 1..=12 {
            dp[i][j] = if i == j {
                1
            } else {
                (j - 1..i).map(|k| dp[k][j - 1]).sum()
            };
        }
    }
    println!("{}", dp[l][12]);
}
