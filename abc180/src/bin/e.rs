use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xyz: [(i32, i32, i32); n],
    }
    let mut costs = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            costs[i][j] = (xyz[j].0 - xyz[i].0).abs()
                + (xyz[j].1 - xyz[i].1).abs()
                + 0.max(xyz[j].2 - xyz[i].2);
        }
    }
    let mut dp = vec![vec![None; n]; 1 << n];
    dp[0][0] = Some(0);
    for i in 0..1 << n {
        for j in 0..n {
            if i >> j & 1 != 0 {
                for k in 0..n {
                    if let Some(m) = dp[i - (1 << j)][k] {
                        dp[i][j] =
                            Some(dp[i][j].map_or(m + costs[k][j], |min| min.min(m + costs[k][j])));
                    }
                }
            }
        }
    }
    println!("{}", dp[(1 << n) - 1][0].unwrap());
}
