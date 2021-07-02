use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: u8, w: u8,
    }
    println!("{}", if s > w { "safe" } else { "unsafe" });
}
