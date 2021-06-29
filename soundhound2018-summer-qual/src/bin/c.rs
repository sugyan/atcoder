use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: f64, m: f64, d: f64,
    }
    println!(
        "{:.10}",
        if d == 0.0 { n } else { (n - d) * 2.0 } * (m - 1.0) / n / n
    );
}
