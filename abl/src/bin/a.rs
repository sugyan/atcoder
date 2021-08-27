use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
    }
    println!("{}", vec!["ACL"; k].join(""));
}
