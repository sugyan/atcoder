use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64, k: i64,
    }
    let k = k.abs();
    let m = |i: i64| n - (n + 1 - i).abs();
    let answer = (k + 2..=n * 2).map(|x| m(x) * m(x - k)).sum::<i64>();
    println!("{}", answer);
}
