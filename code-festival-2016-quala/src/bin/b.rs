use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let c = a
        .iter()
        .enumerate()
        .filter(|&(i, &b)| a[b - 1] - 1 == i)
        .count();
    println!("{}", c / 2);
}
