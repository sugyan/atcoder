use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: f64,
    }
    println!("{}", x.round());
}
