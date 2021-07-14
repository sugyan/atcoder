use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32, b: u32, x: u32, y: u32,
    }
    let answer = x + y.min(x * 2) * if a < b { b - a } else { (a - b).max(1) - 1 };
    println!("{}", answer);
}
