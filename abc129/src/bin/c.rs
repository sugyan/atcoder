use proconio::input;

const MOD: i32 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut broken = vec![false; n + 1];
    for &i in &a {
        broken[i] = true;
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        dp[i] = if broken[i] {
            0
        } else if i == 1 {
            dp[0]
        } else {
            (dp[i - 1] + dp[i - 2]) % MOD
        };
    }
    println!("{}", dp[n]);
}
