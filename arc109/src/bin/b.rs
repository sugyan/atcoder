use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128,
    }
    let (mut lo, mut hi) = (0, n + 1);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if mid * (mid + 1) / 2 > n + 1 {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    println!("{}", n + 2 - hi);
}
