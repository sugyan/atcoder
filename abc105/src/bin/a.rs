use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8, k: u8
    }
    println!("{}", if n % k == 0 { 0 } else { 1 });
}
