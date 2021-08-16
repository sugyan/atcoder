use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let mut answer = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            if (0..4).filter(|&k| s[i + k / 2][j + k % 2] == '#').count() & 1 > 0 {
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}
