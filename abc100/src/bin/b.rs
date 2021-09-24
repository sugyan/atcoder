use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: u32, n: u32,
    }
    println!("{}", (n + n / 100) * 100_u32.pow(d));
}
