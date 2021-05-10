use proconio::marker::Bytes;
use proconio::{fastout, input};

const DIV: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Bytes; 2],
    }
    let mut answer = if s[0][0] != s[1][0] { 6 } else { 3 };
    for i in 1..n {
        if s[0][i] == s[1][i] && s[0][i - 1] == s[1][i - 1] {
            answer = answer * 2 % DIV;
        }
        if i < n - 1 && s[0][i] == s[0][i + 1] {
            answer = answer * if s[0][i - 1] == s[1][i - 1] { 2 } else { 3 } % DIV;
        }
    }
    println!("{}", answer)
}
