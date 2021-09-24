use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u8, b: u8
    }
    println!("{}", if a <= 8 && b <= 8 { "Yay!" } else { ":(" });
}
