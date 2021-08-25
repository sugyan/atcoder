use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i8,
    }
    println!("{}", x.max(0));
}
