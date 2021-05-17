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
                answer += (j - i) as i64;
            } else {
                answer = -1;
                break;
            }
        }
    }
    if s[n - 1] != t[n - 1] {
        answer = -1;
    }
    println!("{}", answer);
}
