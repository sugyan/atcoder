use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }
    let dot = (0..n).map(|i| a[i] * b[i]).sum::<i32>();
    println!("{}", if dot == 0 { "Yes" } else { "No" });
}
