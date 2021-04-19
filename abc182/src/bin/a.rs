use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize, b: usize,
    }
    println!("{}", 2 * a + 100 - b);
}
