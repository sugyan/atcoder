use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32, m: i32, k: i32,
    }
    let answer = (0..=n).any(|i| (0..=m).any(|j| i * m + j * n - 2 * i * j == k));
    println!("{}", if answer { "Yes" } else { "No" });
}
