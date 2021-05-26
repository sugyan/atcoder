use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    }
    let answer = ab
        .iter()
        .map(|&(a, b)| (a + b) * (b - a + 1) / 2)
        .sum::<u64>();
    println!("{}", answer);
}
