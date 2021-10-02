use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    let mut v = vec![0; 10];
    let mut answer = 0;
    while v.iter().rev().fold(0, |acc, &d| acc * 10 + d) <= n {
        if [3, 5, 7].iter().all(|d| v.contains(d)) {
            answer += 1;
        }
        for d in v.iter_mut() {
            if *d == 7 {
                *d = 3
            } else {
                *d += if *d == 0 { 3 } else { 2 };
                break;
            }
        }
    }
    println!("{}", answer);
}
