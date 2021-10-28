use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u8,
    }
    println!("{}", if x == 0 { 1 } else { 0 });
}
