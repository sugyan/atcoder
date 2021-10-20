use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    println!("{}", if &s == "ABC" { "ARC" } else { "ABC" });
}
