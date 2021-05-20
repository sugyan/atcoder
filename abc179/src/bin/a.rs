use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    println!("{}{}", s, if s.ends_with('s') { "es" } else { "s" });
}
