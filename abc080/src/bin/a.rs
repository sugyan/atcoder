use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32, a: u32, b: u32,
    }
    println!("{}", b.min(n * a));
}
