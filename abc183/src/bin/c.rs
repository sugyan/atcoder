use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: u32,
        t: [[u32; n]; n],
    }
    let answer = (0..n)
        .permutations(n)
        .filter(|p| (0..n).map(|i| t[p[i]][p[(i + 1) % n]]).sum::<u32>() == k)
        .count()
        / n;
    println!("{}", answer);
}
