use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
    }
    let answer = (x - 1) / 11 * 2 + if (x - 1) % 11 > 5 { 2 } else { 1 };
    println!("{}", answer);
}
