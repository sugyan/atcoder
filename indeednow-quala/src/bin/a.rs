use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: String,
        b: String,
    }
    println!("{}", a.len() * b.len());
}
