use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
        w: usize,
    }
    let answer = (0..s.len()).step_by(w).map(|i| s[i]).collect::<String>();
    println!("{}", answer);
}
