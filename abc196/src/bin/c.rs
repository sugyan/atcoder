use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: String,
    }
    let len = n.len();
    let answer = if len % 2 != 0 {
        10_u32.pow(len as u32 / 2) - 1
    } else {
        n[..len / 2].parse::<u32>().unwrap() - if n[..len / 2] > n[len / 2..] { 1 } else { 0 }
    };
    println!("{}", answer);
}
