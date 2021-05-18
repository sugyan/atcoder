use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        y: u32,
    }
    let is_leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    println!("{}", if is_leap { "YES" } else { "NO" });
}
