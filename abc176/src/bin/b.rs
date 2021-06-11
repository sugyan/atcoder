use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let sum = s.as_bytes().iter().map(|u| (u - b'0') as u32).sum::<u32>();
    println!("{}", if sum % 9 == 0 { "Yes" } else { "No" });
}
