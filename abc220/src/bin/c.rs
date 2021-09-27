use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        x: u64,
    }
    (1..n).for_each(|i| a[i] += a[i - 1]);
    let mut answer = x / a[n - 1] * n as u64;
    answer += a.iter().position(|&val| val > x % a[n - 1]).unwrap_or(n) as u64 + 1;
    println!("{}", answer);
}
