use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        b: [u32; n - 1],
    }
    let answer = b[0] + b[n - 2] + (1..n - 1).map(|i| b[i].min(b[i - 1])).sum::<u32>();
    println!("{}", answer);
}
