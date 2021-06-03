use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let s0 = (0..n).filter(|&i| s[i] == '0').collect::<Vec<_>>();
    let t0 = (0..n).filter(|&i| t[i] == '0').collect::<Vec<_>>();
    let answer = s0.iter().zip(t0.iter()).filter(|i| i.0 != i.1).count() as i32;
    println!("{}", if s0.len() == t0.len() { answer } else { -1 });
}
