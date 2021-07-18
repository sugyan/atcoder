use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h1: u32, m1: u32, h2: u32, m2: u32, k: u32,
    }
    println!("{}", (h2 - h1) * 60 + m2 - m1 - k);
}
