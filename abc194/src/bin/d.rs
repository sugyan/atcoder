use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!("{:.10}", (1..n).map(|x| n as f64 / x as f64).sum::<f64>());
}
