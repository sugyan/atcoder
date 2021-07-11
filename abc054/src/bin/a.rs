use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        a: u8, b: u8,
    }
    let answer = match ((a + 11) % 13).cmp(&((b + 11) % 13)) {
        Ordering::Less => "Bob",
        Ordering::Equal => "Draw",
        Ordering::Greater => "Alice",
    };
    println!("{}", answer);
}
