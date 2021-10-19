use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u8, b: u8,
    }
    let answer = (0..a.max(b))
        .map(|_| (a.min(b) + b'0') as char)
        .collect::<String>();
    println!("{}", answer);
}
