use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, mut k: u64,
        a: [u32; n],
    }
    let mut b = a.clone();
    b.sort_unstable();
    for &a in &a {
        let answer = k / n as u64 + if a < b[k as usize % n] { 1 } else { 0 };
        println!("{}", answer);
    }
}
