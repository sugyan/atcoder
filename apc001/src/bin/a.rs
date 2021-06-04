use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64, y: i64,
    }
    println!("{}", if x % y != 0 { x } else { -1 });
}
