use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let no = s.contains('N') ^ s.contains('S') || s.contains('E') ^ s.contains('W');
    println!("{}", if !no { "Yes" } else { "No" });
}
