use proconio::{fastout, input};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        a: u64, b: u64, c: u64,
    }
    let answer = [a, b, c]
        .iter()
        .fold(1, |acc, x| acc * ((x * (x + 1) / 2) % MOD) % MOD);
    println!("{}", answer);
}
