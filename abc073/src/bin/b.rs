use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        lr: [(u32, u32); n],
    }
    let answer = lr.iter().map(|&(l, r)| r - l + 1).sum::<u32>();
    println!("{}", answer);
}
