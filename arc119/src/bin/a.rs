use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut answer = n;
    for b in (0..64).filter(|&b| 1 << b <= n) {
        answer = answer.min(n / (1 << b) + b + n - n / (1 << b) * (1 << b));
    }
    println!("{}", answer);
}
