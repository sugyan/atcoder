use num_integer::lcm;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: [u64; n],
    }
    println!("{}", t.into_iter().fold(1, lcm));
}
