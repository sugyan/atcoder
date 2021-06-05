use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [String; 3],
    }
    println!("A{}C", &s[1][..1].to_uppercase());
}
