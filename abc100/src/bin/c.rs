use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    println!("{}", a.iter().map(|a| a.trailing_zeros()).sum::<u32>());
}
