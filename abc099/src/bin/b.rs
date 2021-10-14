use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32, b: u32,
    }
    println!("{}", (b - a) * (b - a + 1) / 2 - b);
}
