use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [u32; n],
    }
    let mut index = vec![0; n];
    for (i, &p) in p.iter().enumerate() {
        index[p as usize - 1] = i;
    }
    let mut dp = vec![1; n];
    let mut max = 1;
    for i in 1..n {
        if index[i - 1] < index[i] {
            dp[i] = dp[i - 1] + 1;
            max = max.max(dp[i]);
        }
    }
    println!("{}", n - max);
}
