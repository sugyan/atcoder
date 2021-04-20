use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { w: u32, h: u32 }
    let answer = if w / 4 * 3 == h { "4:3" } else { "16:9" };
    println!("{}", answer);
}
