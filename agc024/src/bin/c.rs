use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    if a[0] != 0 || a.windows(2).any(|w| w[0] + 1 < w[1]) {
        println!("-1");
        return;
    }
    let mut answer = 0;
    for i in (0..n).rev() {
        if i == n - 1 || a[i] + 1 != a[i + 1] {
            answer += a[i];
        }
    }
    println!("{}", answer);
}
