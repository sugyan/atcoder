use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let mut v = Vec::new();
    for i in 1..=k {
        for s in s.windows(i) {
            v.push(s.iter().collect::<String>());
        }
    }
    v.sort_unstable();
    v.dedup();
    println!("{}", v[k - 1]);
}
