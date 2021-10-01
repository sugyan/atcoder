use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let answer = (0..s.len() - 2)
        .map(|i| (s[i..i + 3].parse::<i32>().unwrap() - 753).abs())
        .min()
        .unwrap();
    println!("{}", answer);
}
