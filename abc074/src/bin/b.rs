use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u32,
        x: [u32; n],
    }
    println!("{}", x.iter().map(|&x| x.min(k - x) * 2).sum::<u32>());
}
