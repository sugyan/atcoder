use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u8,
    }
    println!("{}", if matches!(x, 7 | 5 | 3) { "YES" } else { "NO" });
}
