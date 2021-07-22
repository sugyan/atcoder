use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        a: u32,
    }
    println!("{}", n * n - a);
}
