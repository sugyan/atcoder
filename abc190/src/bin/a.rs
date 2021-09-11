use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u8, b: u8, c: u8,
    }
    let t = a > b || (a == b && c == 1);
    println!("{}", if t { "Takahashi" } else { "Aoki" });
}
