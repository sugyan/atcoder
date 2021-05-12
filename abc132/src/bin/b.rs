use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [u8; n],
    }
    let answer = p
        .windows(3)
        .filter(|&w| (w[0] > w[1] && w[1] > w[2]) || (w[0] < w[1] && w[1] < w[2]))
        .count();
    println!("{}", answer);
}
