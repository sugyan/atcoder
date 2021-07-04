use num_integer::gcd;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64, b: u64, c: u64,
    }
    let gcd = [a, b, c].iter().fold(a, |acc, &x| gcd(acc, x));
    println!("{}", a / gcd + b / gcd + c / gcd - 3);
}
