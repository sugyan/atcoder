use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: String,
    }
    println!("{}", x.split('.').next().unwrap());
}
