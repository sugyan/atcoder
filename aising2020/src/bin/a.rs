use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: u8, r: u8, d: u8,
    }
    println!("{}", (l..=r).filter(|e| e % d == 0).count());
}
