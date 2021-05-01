use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        mut h: [u32; n],
    }
    h.sort_unstable();
    let answer = h.windows(k).map(|w| w[k - 1] - w[0]).min().unwrap();
    println!("{}", answer);
}
