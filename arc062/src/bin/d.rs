use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    println!("{}", s.len() / 2 - s.chars().filter(|&c| c == 'p').count());
}
