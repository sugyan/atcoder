use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        k: u64,
    }
    let answer = match s.chars().enumerate().find(|&(_, c)| c != '1') {
        Some((i, c)) if k > i as u64 => c,
        _ => '1',
    };
    println!("{}", answer);
}
