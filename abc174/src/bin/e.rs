use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: u32,
        a: [u32; n],
    }
    let (mut lo, mut hi) = (0, 1_000_000_000);
    while lo + 1 < hi {
        let m = lo + (hi - lo) / 2;
        if a.iter().map(|a| (a - 1) / m).sum::<u32>() <= k {
            hi = m;
        } else {
            lo = m;
        }
    }
    println!("{}", hi);
}
