use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    let r = s.iter().filter(|&c| *c == 'R').count();
    println!("{}", (0..r).filter(|&i| s[i] == 'W').count());
}
