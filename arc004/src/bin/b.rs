use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [u32; n],
    }
    let sum = d.iter().sum::<u32>();
    let max = *d.iter().max().unwrap();
    let min = if max * 2 > sum { max * 2 - sum } else { 0 };
    println!("{}", sum);
    println!("{}", min);
}
