use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut answer = 0;
    for i in 1..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            if i & 1 == 0 && (n / i) & 1 == 0 {
                continue;
            }
            answer += if i * i == n { 1 } else { 2 };
        }
    }
    println!("{}", answer * if n & 1 == 0 { 1 } else { 2 });
}
