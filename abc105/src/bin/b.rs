use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8,
    }
    let answer = (0..=n).step_by(7).any(|m| (n - m) % 4 == 0);
    println!("{}", if answer { "Yes" } else { "No" });
}
