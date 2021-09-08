use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        k: usize,
    }
    if let Some((_, c)) = s.chars().enumerate().find(|&(i, c)| i + 1 == k || c != '1') {
        println!("{}", c);
    }
}
