use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    println!("{}", (1..n).map(|i| (n - 1) / i).sum::<u32>());
}
