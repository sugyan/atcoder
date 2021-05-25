use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        m: u32,
    }
    let answer = match m {
        0..=99 => 0,
        100..=5000 => m / 100,
        6000..=30000 => m / 1000 + 50,
        35000..=70000 => (m / 1000 - 30) / 5 + 80,
        _ => 89,
    };
    println!("{:02}", answer);
}
