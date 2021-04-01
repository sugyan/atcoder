use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        a: [Chars; h],
    }
    if a.iter().flatten().filter(|&c| *c == '#').count() == h + w - 1 {
        println!("Possible");
    } else {
        println!("Impossible");
    }
}
