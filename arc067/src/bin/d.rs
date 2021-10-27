use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, a: u64, b: u64,
        x: [u64; n],
    }
    let answer = (1..n).map(|i| b.min(a * (x[i] - x[i - 1]))).sum::<u64>();
    println!("{}", answer);
}
