use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let mut sum = vec![vec![0; w]; h];
    for i in 0..h {
        let mut len = 0;
        {
            for j in 0..w {
                len = if s[i][j] == '#' { 0 } else { len + 1 };
                sum[i][j] += len;
            }
        }
        len = 0;
        {
            for j in (0..w).rev() {
                len = if s[i][j] == '#' { 0 } else { len + 1 };
                sum[i][j] += len;
            }
        }
    }
    for j in 0..w {
        let mut len = 0;
        {
            for i in 0..h {
                len = if s[i][j] == '#' { 0 } else { len + 1 };
                sum[i][j] += len;
            }
        }
        len = 0;
        {
            for i in (0..h).rev() {
                len = if s[i][j] == '#' { 0 } else { len + 1 };
                sum[i][j] += len;
            }
        }
    }
    println!(
        "{}",
        sum.iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap()
            - 3
    );
}
