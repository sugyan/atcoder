use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: u64, n: u64,
    }
    println!("{}", (100 * n - 1) / t + n);
}
