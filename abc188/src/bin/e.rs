use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [i32; n],
        mut xy: [(usize, usize); m],
    }
    let mut dp = vec![std::i32::MIN; n];
    xy.sort();
    for &(x, y) in &xy {
        dp[y - 1] = dp[y - 1].max(dp[x - 1].max(0) + a[y - 1] - a[x - 1]);
    }
    println!("{}", dp.iter().max().unwrap());
}
