use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: u32, mut b: u32, k: usize,
    }
    for i in 0..k {
        if i % 2 == 0 {
            a /= 2;
            b += a;
        } else {
            b /= 2;
            a += b;
        }
    }
    println!("{} {}", a, b);
}
