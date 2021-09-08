use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut answer = 0;
    for i in (1..=n).step_by(2) {
        if (1..=i).filter(|&j| i % j == 0).count() == 8 {
            answer += 1;
        }
    }
    println!("{}", answer);
}
