use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    for i in 0..n {
        for j in 0..n {
            if (i < n - 5 && (0..6).filter(|k| s[i + k][j] == '#').count() >= 4)
                || (j < n - 5 && (0..6).filter(|k| s[i][j + k] == '#').count() >= 4)
                || (i < n - 5
                    && j < n - 5
                    && (0..6).filter(|k| s[i + k][j + k] == '#').count() >= 4)
                || (i < n - 5
                    && j < n - 5
                    && (0..6).filter(|k| s[i + 5 - k][j + k] == '#').count() >= 4)
            {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
