use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let mut v = (Vec::new(), Vec::new());
    for i in 0..n {
        if s[i] == '0' {
            v.0.push(i);
        }
        if t[i] == '0' {
            v.1.push(i);
        }
    }
    let answer = if v.0.len() != v.1.len() {
        -1
    } else {
        v.0.iter().zip(v.1.iter()).filter(|i| i.0 != i.1).count() as i32
    };
    println!("{}", answer);
}
