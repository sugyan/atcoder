use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64, b: u64, c: u64, d: u64,
    }
    println!("{}", if a > d || b < c { "No" } else { "Yes" });
}
