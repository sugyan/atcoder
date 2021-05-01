use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, x: u64
    }
    let (mut p, mut len) = (1_u64, 1);
    for _ in 1..=n {
        p = p * 2 + 1;
        len = len * 2 + 3;
    }
    let (mut lo, mut hi) = (0, len - 1);
    let mut answer = 0;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        p /= 2;
        if x > mid {
            answer += p + 1;
            lo = mid + 1;
            hi -= 1;
        } else {
            lo += 1;
            hi = mid - 1;
        }
    }
    println!("{}", answer);
}
