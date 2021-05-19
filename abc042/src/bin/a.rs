use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u8, b: u8, c: u8,
    }
    let mut v = vec![a, b, c];
    v.sort_unstable();
    println!("{}", if v == vec![5, 5, 7] { "YES" } else { "NO" });
}
