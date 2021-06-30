use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32, l: u32,
        k: usize,
        mut a: [u32; n],
    }
    a.push(l);
    let (mut lo, mut hi) = (0, l);
    while lo + 1 < hi {
        let m = (lo + hi) / 2;
        if available(&a, k, m) {
            lo = m;
        } else {
            hi = m;
        }
    }
    println!("{}", lo);
}

fn available(a: &[u32], k: usize, target: u32) -> bool {
    let (mut count, mut prev) = (0, 0);
    for &a in a {
        if a - prev >= target {
            count += 1;
            prev = a;
        }
    }
    count > k
}
