use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [i32; n],
    }
    let sum = d.iter().sum::<i32>();
    let max = *d.iter().max().unwrap();
    let min = (max * 2 - sum).max(0);
    println!("{}", sum);
    println!("{}", min);
}
