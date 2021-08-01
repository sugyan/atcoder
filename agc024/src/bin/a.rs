use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64, b: i64, _c: i64, k: u64,
    }
    println!("{}", if k & 1 == 0 { a - b } else { b - a });
}
