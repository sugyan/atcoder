use proconio::input;

const MOD: i32 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut dp = vec![None; n + 1];
    for &a in &a {
        dp[a] = Some(0);
    }
    dp[0] = Some(1);
    for i in 1..=n {
        if dp[i] == Some(0) {
            continue;
        }
        let mut val = dp[i - 1].unwrap_or_default();
        if i > 1 {
            val += dp[i - 2].unwrap_or_default();
        }
        dp[i] = Some(val % MOD);
    }
    println!("{}", dp[n].unwrap());
}
