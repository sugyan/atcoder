use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x: u128, y: u128, a: u128, b: u128,
    }
    let mut answer = 0;
    while x * a < y && x * a < x + b {
        x *= a;
        answer += 1;
    }
    answer += (y - x - 1) / b;
    println!("{}", answer);
}
