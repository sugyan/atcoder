use proconio::marker::Bytes;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut h: usize, mut w: usize,
        a: [Bytes; h],
    }
    let mut counts = [0; 26];
    for row in a {
        row.iter().for_each(|u| counts[(u - b'a') as usize] += 1);
    }
    let mut available = || {
        for _ in 0..(h / 2) * (w / 2) {
            if let Some(c) = counts.iter_mut().find(|c| **c >= 4) {
                *c -= 4;
            } else {
                return false;
            }
        }
        for _ in 0..(h % 2) * w / 2 + (w % 2) * h / 2 {
            if let Some(c) = counts.iter_mut().find(|c| **c >= 2) {
                *c -= 2;
            } else {
                return false;
            }
        }
        true
    };
    println!("{}", if available() { "Yes" } else { "No" });
}
