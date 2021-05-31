use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u8, b: u8,
    }
    println!("{}", if a * b % 2 != 0 { "Yes" } else { "No" });
}
