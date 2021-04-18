use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, r: usize,
        mut s: Chars,
    }
    let mut i = n - r;
    while i > 0 && s[i + r - 1] == 'o' {
        i -= 1;
    }
    let mut answer = i;
    while i > 0 {
        answer += 1;
        for j in 0..r {
            s[i + j] = 'o';
        }
        while i > 0 && s[i + r - 1] == 'o' {
            i -= 1;
        }
    }
    if (0..r).any(|j| s[j] == '.') {
        answer += 1;
    }
    println!("{}", answer);
}
