use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    println!("AB{}", if n > 999 { 'D' } else { 'C' });
}
