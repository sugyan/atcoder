use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: String,
        k: usize,
    }
    let n = n.bytes().map(|u| (u - b'0') as u32).collect::<Vec<_>>();
    let mut dp = vec![vec![0; 100]; 3];
    dp[0][0] = 9;
    for i in 1..100 {
        for j in 0..3 {
            dp[j][i] = dp[j][i - 1] + 9 * if j > 0 { dp[j - 1][i - 1] } else { 1 };
        }
    }
    println!("{}", helper(&dp, &n, k));
}

fn helper(dp: &[Vec<u32>], n: &[u32], k: usize) -> u32 {
    if n.is_empty() {
        return 0;
    }
    if n[0] == 0 {
        return helper(dp, &n[1..], k);
    }
    if k == 1 {
        return n[0] + (n.len() as u32 - 1) * 9;
    }
    if n.len() < 2 {
        return 0;
    }
    dp[k - 1][n.len() - 2] + (n[0].max(1) - 1) * dp[k - 2][n.len() - 2] + helper(dp, &n[1..], k - 1)
}
