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
    for i in 1..=m {
        println!("{} {}", i * 2, i * 2 + 1);
    }
    println!("{} {}", (m + 1) * 2, (m + 2) * 2);
    if n == 1 {
        return;
    }
    println!("1 {}", (m + 1) * 2 + 1);
    for i in 0..n - m - 2 {
        println!("{} {}", (m + 2 + i) * 2 + 1, (m + 3 + i) * 2);
    }
}
