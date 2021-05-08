use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut sieve = vec![true; n + 1];
    for i in 2..n {
        if sieve[i] {
            for j in (2..).map(|j| j * i).take_while(|&j| j <= n) {
                sieve[j] = false
            }
        }
    }
    println!("{}", if sieve[n] { "YES" } else { "NO" });
}
