use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32, b: u32,
    }
    println!("{}", (a - 1) * (b - 1));
}
