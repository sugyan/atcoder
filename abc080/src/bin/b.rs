use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    let sum: u8 = n.to_string().as_bytes().iter().map(|&b| b - b'0').sum();
    println!("{}", if n % u32::from(sum) == 0 { "Yes" } else { "No" });
}
