use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64, k: i64,
    }
    let mut answer = 0;
    let m = |i: i64| n - (n + 1 - i).abs();
    for i in 2..=(n * 2) {
        if (k + 2..=n * 2 + k).contains(&i) {
            answer += m(i) * m(i - k);
        }
    }
    println!("{}", answer);
}
