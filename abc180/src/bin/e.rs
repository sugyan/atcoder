use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xyz: [(i32, i32, i32); n],
    }
    let mut dp = vec![vec![std::i32::MAX; n]; 1 << n];
    dp[0][0] = 0;
    for i in 0..1 << n {
        for j in 0..n {
            if i >> j & 1 == 0 {
                continue;
            }
            for k in 0..n {
                let c = (xyz[k].0 - xyz[j].0).abs()
                    + (xyz[k].1 - xyz[j].1).abs()
                    + 0.max(xyz[k].2 - xyz[j].2);
                dp[i][j] = dp[i][j].min(dp[i - (1 << j)][k].saturating_add(c));
            }
        }
    }
    println!("{}", dp[(1 << n) - 1][0]);
}
