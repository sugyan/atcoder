use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[u64; 6]; n],
    }
    let answer = a.iter().fold(1, |acc, v| acc * v.iter().sum::<u64>() % MOD);
    println!("{}", answer);
}
