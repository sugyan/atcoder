use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String, t: String,
        a: u8, b: u8,
        u: String,
    }
    println!(
        "{} {}",
        a - if u == s { 1 } else { 0 },
        b - if u == t { 1 } else { 0 }
    );
}
