use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64, b: u64, x: u64,
    }
    println!("{}", b / x - a / x + if a % x == 0 { 1 } else { 0 });
}
