use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut answer = 0;
    let (mut lo, mut hi) = (0, n - 1);
    while lo < hi {
        while lo < hi && s[lo] == 'R' {
            lo += 1;
        }
        while lo < hi && s[hi] == 'W' {
            hi -= 1;
        }
        if lo < hi {
            answer += 1;
            lo += 1;
            hi -= 1;
        }
    }
    println!("{}", answer);
}
