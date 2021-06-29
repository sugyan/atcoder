use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }
    a.sort_unstable();
    b.sort_unstable();
    let answer = a.iter().zip(&b).map(|(a, b)| (a - b).abs()).sum::<i64>();
    println!("{}", answer);
}
