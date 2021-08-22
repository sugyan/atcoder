use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        p: [u32; n],
    }
    let mut sum = (0..k).map(|i| p[i]).sum::<u32>();
    let mut max = sum;
    for i in 0..n - k {
        sum = sum + p[i + k] - p[i];
        max = max.max(sum);
    }
    println!("{:.10}", (max + k as u32) as f64 / 2.0);
}
