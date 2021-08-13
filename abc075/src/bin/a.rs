use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i8, b: i8, c: i8,
    }
    println!("{}", a ^ b ^ c);
}
