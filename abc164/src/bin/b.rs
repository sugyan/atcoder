use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: i8, b: i8, mut c: i8, d: i8,
    }
    while a > 0 && c > 0 {
        a -= d;
        c -= b;
    }
    println!("{}", if c <= 0 { "Yes" } else { "No" });
}
