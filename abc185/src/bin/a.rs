use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [u8; 4],
    }
    println!("{}", a.iter().min().unwrap());
}
