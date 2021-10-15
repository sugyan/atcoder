use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u8; n],
    }
    let answer = a
        .iter()
        .enumerate()
        .filter(|&(i, &e)| i % 2 == 0 && e % 2 != 0)
        .count();
    println!("{}", answer);
}
