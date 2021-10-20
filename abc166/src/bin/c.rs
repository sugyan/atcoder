use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        h: [u32; n],
        ab: [(usize, usize); m],
    }
    let mut v = vec![true; n];
    for &(a, b) in &ab {
        if h[a - 1] <= h[b - 1] {
            v[a - 1] = false;
        }
        if h[a - 1] >= h[b - 1] {
            v[b - 1] = false;
        }
    }
    println!("{:?}", v.iter().filter(|&b| *b).count());
}
