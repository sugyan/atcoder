use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [(u8, u8); n],
    }
    let b = d.windows(3).any(|w| w.iter().all(|d| d.0 == d.1));
    println!("{}", if b { "Yes" } else { "No" });
}
