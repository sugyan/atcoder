use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32,
        s: String,
    }
    println!("{}", if a >= 3200 { &s } else { "red" });
}
