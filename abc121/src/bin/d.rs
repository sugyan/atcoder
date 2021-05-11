use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: i64, b: i64,
    }
    let f = |n: i64| (n + 1) >> 1 & 1 ^ if n & 1 > 0 { 0 } else { n };
    println!("{}", f(b) ^ f(a - 1));
}
