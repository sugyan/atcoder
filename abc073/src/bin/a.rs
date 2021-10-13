use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: String,
    }
    println!("{}", if n.contains('9') { "Yes" } else { "No" });
}
