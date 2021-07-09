use num_integer::gcd;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64, b: u64,
    }
    let g = gcd(a, b);
    if a / g > 10_u64.pow(18) / b {
        println!("Large");
    } else {
        println!("{}", a / g * b);
    }
}
