use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let answer = (1..1_000_000)
        .filter(|i| format!("{}{}", i, i).parse::<u64>().unwrap() <= n)
        .count();
    println!("{}", answer);
}
