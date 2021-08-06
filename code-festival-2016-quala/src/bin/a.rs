use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    println!("{} {}", &s[..4], &s[4..]);
}
