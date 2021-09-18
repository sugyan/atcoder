use proconio::{fastout, input};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut answer = 0;
    for i in (1..).take_while(|&i| i * i <= n) {
        answer = (answer + (n - i * i) / i / 2 + 1) % MOD;
    }
    println!("{}", answer);
}
