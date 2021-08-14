use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: i32, t: i32,
    }
    let mut answer = 0;
    for a in 0..=s {
        for b in 0..=s - a {
            for c in 0..=s - a - b {
                if a * b * c <= t {
                    answer += 1;
                }
            }
        }
    }
    println!("{}", answer);
}
