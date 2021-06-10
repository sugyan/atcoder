use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: i32,
    }
    let mut s = Vec::new();
    while n != 0 {
        s.push(if n % 2 != 0 { '1' } else { '0' });
        n = (n - (n & 1)) / -2;
    }
    let answer = s.iter().rev().collect::<String>();
    println!("{}", if answer.is_empty() { "0" } else { &answer });
}
