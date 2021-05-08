use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    println!("{}", (n - 1) / 100 + 1);
}
