use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: u32,
        a: String, b: String,
    }
    let answer = u64::from_str_radix(&a, k).unwrap() * u64::from_str_radix(&b, k).unwrap();
    println!("{}", answer);
}
