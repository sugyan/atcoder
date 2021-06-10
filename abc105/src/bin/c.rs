use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: i32,
    }
    let mut s = Vec::new();
    while n != 0 {
        s.push(if n % 2 != 0 {
            n -= 1;
            '1'
        } else {
            '0'
        });
        n /= -2;
    }
    if s.is_empty() {
        s.push('0');
    }
    println!("{}", s.iter().rev().collect::<String>());
}
