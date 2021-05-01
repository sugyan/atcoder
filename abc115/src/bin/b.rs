use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [u32; n],
    }
    let answer = p.iter().sum::<u32>() - p.iter().max().unwrap() / 2;
    println!("{}", answer);
}
