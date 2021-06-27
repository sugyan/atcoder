use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8, k: u8,
    }
    println!("{}", if k == 1 { 0 } else { n - k });
}
