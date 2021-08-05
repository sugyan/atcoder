use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!("{}", n * (n - 1) / 2 - n / 2);
    for i in 1..=n {
        for j in i + 1..=n {
            if i + j != n + 1 - n % 2 {
                println!("{} {}", i, j);
            }
        }
    }
}
