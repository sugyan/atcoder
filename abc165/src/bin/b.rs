use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
    }
    let (mut answer, mut m) = (0, 100);
    while m < x {
        m += m / 100;
        answer += 1;
    }
    println!("{}", answer);
}
