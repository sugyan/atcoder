use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    for (i, a) in a.iter_mut().enumerate() {
        *a -= i as i64 + 1;
    }
    a.sort_unstable();
    println!("{}", a.iter().map(|&b| (b - a[n / 2]).abs()).sum::<i64>());
}
