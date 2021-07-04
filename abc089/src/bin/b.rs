use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [char; n],
    }
    let four = s.iter().any(|&c| c == 'Y');
    println!("{}", if four { "Four" } else { "Three" });
}
