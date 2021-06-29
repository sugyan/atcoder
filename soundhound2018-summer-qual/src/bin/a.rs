use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u8, b: u8
    }
    let answer = if a + b == 15 {
        "+"
    } else if a * b == 15 {
        "*"
    } else {
        "x"
    };
    println!("{}", answer);
}
