use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64, k: u64,
    }
    let mut answer = 0;
    for b in k + 1..=n {
        answer += (b - k) * (n / b);
        if k == 0 {
            answer += n % b;
        } else if n % b >= k {
            answer += n % b - k + 1
        }
    }
    println!("{}", answer);
}
