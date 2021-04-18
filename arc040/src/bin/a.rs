use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let (r, b) = s.iter().fold((0, 0), |mut acc, x| {
        x.chars().for_each(|c| match c {
            'R' => acc.0 += 1,
            'B' => acc.1 += 1,
            _ => {}
        });
        acc
    });
    let answer = match r.cmp(&b) {
        Ordering::Less => "AOKI",
        Ordering::Equal => "DRAW",
        Ordering::Greater => "TAKAHASHI",
    };
    println!("{}", answer);
}
