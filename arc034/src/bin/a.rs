use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        scores: [[f64; 5]; n]
    }
    let mut answer = 0_f64;
    for s in &scores {
        answer = answer.max(s[0] + s[1] + s[2] + s[3] + s[4] * 110.0 / 900.0);
    }
    println!("{}", answer);
}
