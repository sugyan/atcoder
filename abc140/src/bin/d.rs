use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        s: String,
    }
    let answer = (n - 1).min(s.as_bytes().windows(2).filter(|w| w[0] == w[1]).count() + k * 2);
    println!("{}", answer);
}
