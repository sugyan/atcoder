use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32, x: u32, t: u32,
    }
    println!("{}", t * ((n - 1) / x + 1));
}
