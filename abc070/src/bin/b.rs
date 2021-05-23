use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i8, b: i8, c: i8, d: i8,
    }
    println!("{}", 0.max(b.min(d) - a.max(c)));
}
