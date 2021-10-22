use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut min = s.clone();
    let mut max = s.clone();
    for i in 1..s.len() {
        let s = String::from(&s[i..]) + &s[..i];
        min = min.min(s.clone());
        max = max.max(s.clone());
    }
    println!("{}", min);
    println!("{}", max);
}
