use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: u32,
        a: u32, b: u32,
    }
    let answer = if (a..=b).any(|p| p % k == 0) {
        "OK"
    } else {
        "NG"
    };
    println!("{}", answer);
}
