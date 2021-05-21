use proconio::{fastout, input};

const DIV: i64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        lr: [(usize, usize); k],
    }
    let mut v = vec![Vec::new(); n];
    for i in 0..n {
        for &(l, r) in &lr {
            if i + l < n {
                v[i + l].push((i, 1));
            }
            if i + r + 1 < n {
                v[i + r + 1].push((i, -1));
            }
        }
    }
    let mut dp = vec![0_i64; n];
    dp[0] = 1;
    let mut d = 0;
    for i in 1..n {
        for &(j, sign) in &v[i] {
            d = (d + dp[j] * sign).rem_euclid(DIV);
        }
        dp[i] = d;
    }
    println!("{}", dp[n - 1]);
}
