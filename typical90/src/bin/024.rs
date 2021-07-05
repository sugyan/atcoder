use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, mut k: i32,
        a: [i32; n],
        b: [i32; n],
    }
    (0..n).for_each(|i| k -= (a[i] - b[i]).abs());
    println!("{}", if k >= 0 && k & 1 == 0 { "Yes" } else { "No" });
}
