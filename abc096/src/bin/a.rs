use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u8, b: u8,
    }
    println!("{}", a - if b < a { 1 } else { 0 });
}
