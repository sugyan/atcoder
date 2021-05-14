use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    println!("{}", if n == 2 { "WANWAN" } else { "BOWWOW" });
}
