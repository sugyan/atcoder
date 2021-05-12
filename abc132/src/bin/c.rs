use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut d: [u32; n],
    }
    d.sort_unstable();
    println!("{}", d[n / 2] - d[n / 2 - 1]);
}
