use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8, a: u8, b: u8,
    }
    println!("{} {}", a.min(b), n.max(a + b) - n);
}
