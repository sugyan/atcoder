use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: u32,
        a: u32, b: u32,
    }
    let answer = if k == 1 || a % k == 0 || b % k == 0 || a / k != b / k {
        "OK"
    } else {
        "NG"
    };
    println!("{}", answer);
}
