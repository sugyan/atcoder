use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut answer = 0;
    for i in 0..10_000 {
        let v = [i % 10, i / 10 % 10, i / 100 % 10, i / 1000];
        if !(0..10).any(|j| (s[j] == 'o' && !v.contains(&j)) || (s[j] == 'x' && v.contains(&j))) {
            answer += 1;
        }
    }
    println!("{}", answer);
}
