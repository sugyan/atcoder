use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: String,
    }
    let s = n.trim_end_matches('0');
    let ok = s.chars().rev().collect::<String>() == s;
    println!("{}", if ok { "Yes" } else { "No" });
}
