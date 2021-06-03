use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut answer = n;
    for b in (0..64).filter(|&b| 1 << b <= n) {
        let a = (n - (1 << b)) / (1 << b) + 1;
        let c = n - a * (1 << b);
        answer = answer.min(a + b + c);
    }
    println!("{}", answer);
}
