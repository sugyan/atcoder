use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        abc: [Chars; 3],
    }
    let mut answer = n * 2;
    for i in 0..n {
        let (a, b, c) = (abc[0][i], abc[1][i], abc[2][i]);
        if a == b && b == c {
            answer -= 2;
        } else if a == b || b == c || a == c {
            answer -= 1;
        }
    }
    println!("{}", answer);
}
