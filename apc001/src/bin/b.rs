use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let t = (0..n)
        .map(|i| a[i] - b[i])
        .map(|d| if d < 0 { d / 2 } else { d })
        .sum::<i64>();
    println!("{}", if t > 0 { "No" } else { "Yes" });
}
