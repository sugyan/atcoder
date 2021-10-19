use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8, m: u8,
    }
    println!("{}", if n == m { "Yes" } else { "No" })
}
