use proconio::{fastout, input};

const MOD: i64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let pow = |x: i64| (0..n).fold(1, |acc, _| acc * x % MOD);
    println!("{}", (pow(10) + pow(8) - 2 * pow(9)).rem_euclid(MOD));
}
