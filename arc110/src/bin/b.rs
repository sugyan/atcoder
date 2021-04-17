use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }
    if s.contains("00") || s.contains("111") || s.contains("010") {
        println!(0);
    } else if &s == "1" {
        println!(20_000_000_000);
    } else if &s == "11" {
        println!(10_000_000_000);
    } else {
        let mut answer = 10_000_000_000 - s.chars().filter(|&c| c == '0').count();
        if s.ends_with('0') {
            answer += 1;
        }
        println!("{}", answer);
    }
}
