use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: Chars,
        t: Chars,
    }
    let mut answer = 0;
    let mut v = (0..n).rev().filter(|&i| s[i] == '1').collect::<Vec<_>>();
    for i in 0..n - 1 {
        if s[i] == '1' {
            v.pop();
        }
        if s[i] != t[i] {
            if let Some(j) = v.pop() {
                s[j] = '0';
                answer += j - i;
            } else {
                break;
            }
            s[i] = t[i];
        }
    }
    println!("{}", if s == t { answer as i64 } else { -1 });
}
