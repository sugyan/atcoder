use proconio::{fastout, input};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let answer = (1..)
        .take_while(|&i| i * i <= n)
        .fold(0, |acc, i| (acc + (n - i * i) / i / 2 + 1) % MOD);
    println!("{}", answer);
}
