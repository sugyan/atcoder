use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i32,
    }
    println!("{}", if x >= 30 { "Yes" } else { "No" });
}
