use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }
    let mut indeednow = "indeednow".chars().collect::<Vec<_>>();
    indeednow.sort_unstable();
    for s in s.iter_mut() {
        s.sort_unstable();
        println!("{}", if *s == indeednow { "YES" } else { "NO" });
    }
}
