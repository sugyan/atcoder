use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        v: u32, t: u32, s: u32, d: u32,
    }
    let answer = if (v * t..=v * s).contains(&d) {
        "No"
    } else {
        "Yes"
    };
    println!("{}", answer);
}
