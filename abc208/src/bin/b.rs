use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut p: u32,
    }
    let mut y = (1..=10).product::<u32>();
    let mut answer = 0;
    for i in (1..=10).rev() {
        answer += p / y;
        p %= y;
        y /= i;
    }
    println!("{}", answer);
}
