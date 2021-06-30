use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: u32,
    }
    let mut n = 0;
    for i in 0..k {
        n = (n * 10 + 7) % k;
        if n == 0 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
