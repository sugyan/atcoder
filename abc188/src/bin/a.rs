use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i8, y: i8,
    }
    println!("{}", if (x - y).abs() < 3 { "Yes" } else { "No" });
}
