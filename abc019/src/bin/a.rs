use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u8, b: u8, c: u8,
    }
    let mut v = [a, b, c];
    v.sort_unstable();
    println!("{}", v[1]);
}
