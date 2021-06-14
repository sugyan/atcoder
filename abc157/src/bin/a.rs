use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8,
    }
    println!("{}", n / 2 + n % 2);
}
