use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: u64,
    }
    let mut answer = String::new();
    for _ in 0..60 {
        if n & 1 > 0 {
            answer.push('A');
        }
        answer.push('B');
        n >>= 1;
    }
    println!("{}", answer.chars().rev().collect::<String>());
}
