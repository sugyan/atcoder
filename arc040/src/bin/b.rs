use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, r: usize,
        mut s: Chars,
    }
    let mut answer = 0;
    if let Some(pos) = s.iter().rev().position(|&c| c == '.') {
        if n - r > pos {
            answer += n - r - pos;
        }
    }
    while let Some(pos) = s.iter().position(|&c| c == '.') {
        answer += 1;
        (0..r).for_each(|i| s[(n - 1).min(pos + i)] = 'o');
    }
    println!("{}", answer);
}
