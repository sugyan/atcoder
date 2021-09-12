use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: u64,
    }
    n *= 2;
    let answer = (1..)
        .take_while(|&i| i * i <= n)
        .filter(|&i| n % i == 0 && i & 1 != (n / i) & 1)
        .count()
        * 2;
    println!("{}", answer);
}
