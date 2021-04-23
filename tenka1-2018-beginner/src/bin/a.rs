use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { s: String }
    let answer = if s.len() == 2 {
        s
    } else {
        s.chars().rev().collect()
    };
    println!("{}", answer);
}
