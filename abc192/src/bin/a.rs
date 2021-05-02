use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u32,
    }
    println!("{}", 100 - x % 100);
}
