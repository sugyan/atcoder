use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [f64; n],
    }
    println!("{}", 1.0 / a.iter().map(|a| 1.0 / a).sum::<f64>());
}
