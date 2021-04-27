use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64, k: u64,
    }
    let mut answer = 0;
    for b in k + 1..=n {
        answer += (b - k) * (n / b) + if n % b >= k { n % b - k + 1 } else { 0 };
    }
    answer -= if k == 0 { n } else { 0 };
    println!("{}", answer);
}
