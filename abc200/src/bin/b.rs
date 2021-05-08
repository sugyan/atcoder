use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: u64, k: usize,
    }
    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = n * 1000 + 200;
        }
    }
    println!("{}", n);
}
