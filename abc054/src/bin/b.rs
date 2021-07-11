use proconio::marker::Bytes;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [Bytes; n],
        b: [Bytes; m],
    }
    for i in 0..=n - m {
        for j in 0..=n - m {
            if (0..m).all(|k| (0..m).all(|l| a[i + k][j + l] == b[k][l])) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
