use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64, b: u64, x: u64,
    }
    let (mut lo, mut hi) = (0, x.min(1_000_000_001));
    while lo < hi {
        let m = (lo + hi) / 2;
        if a * m + b * m.to_string().len() as u64 > x {
            hi = m;
        } else {
            lo = m + 1;
        }
    }
    println!("{}", lo.max(1) - 1);
}
