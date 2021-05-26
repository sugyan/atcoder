use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8,
    }
    println!("{}", if n & 1 == 0 { "White" } else { "Black" });
}
