use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        ab: [(u64, u64); n],
    }
    let mut v = Vec::with_capacity(n * 2);
    for &(a, b) in &ab {
        v.push(b);
        v.push(a - b);
    }
    v.sort_unstable();
    println!("{}", v.iter().skip(n * 2 - k).sum::<u64>());
}
