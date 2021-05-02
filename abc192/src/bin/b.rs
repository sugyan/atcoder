use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let b = s
        .chars()
        .enumerate()
        .all(|(i, c)| (i % 2 == 0) == c.is_lowercase());
    println!("{}", if b { "Yes" } else { "No" });
}
