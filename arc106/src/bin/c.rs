use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32, m: i32,
    }
    if n > 1 && !(0..=n - 2).contains(&m) {
        println!("-1");
        return;
    }
    for i in 0..n - 1 {
        println!("{} {}", i * 3 + 2, i * 3 + 4);
    }
    println!("1 {}", 3 * (m + 1));
}
