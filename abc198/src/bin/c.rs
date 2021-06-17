use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: u64, x: u64, y: u64,
    }
    let d = ((x * x + y * y) as f64).sqrt().ceil() as u64;
    let answer = (d - 1) / r + 1;
    println!(
        "{}",
        if answer == 1 && r != d {
            answer + 1
        } else {
            answer
        }
    );
}
