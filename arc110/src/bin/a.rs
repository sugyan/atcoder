use num_integer::lcm;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: u64 }
    println!("{}", (2..=n).fold(1, lcm) + 1);
}
