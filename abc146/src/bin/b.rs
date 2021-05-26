use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8,
        s: String,
    }
    let answer = s
        .as_bytes()
        .iter()
        .map(|u| (b'A' + (u - b'A' + n) % 26) as char)
        .collect::<String>();
    println!("{}", answer);
}
