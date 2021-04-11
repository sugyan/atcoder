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
        {
            let mut len = 0;
            for j in 0..w {
                if s[i][j] == '#' {
                    len = 0;
                } else {
                    sum[i][j] += len;
                    len += 1;
                }
            }
        }
        {
            let mut len = 0;
            for j in (0..w).rev() {
                if s[i][j] == '#' {
                    len = 0;
                } else {
                    sum[i][j] += len;
                    len += 1;
                }
            }
        }
    }
    for j in 0..w {
        {
            let mut len = 0;
            for i in 0..h {
                if s[i][j] == '#' {
                    len = 0;
                } else {
                    sum[i][j] += len;
                    len += 1;
                }
            }
        }
        {
            let mut len = 0;
            for i in (0..h).rev() {
                if s[i][j] == '#' {
                    len = 0;
                } else {
                    sum[i][j] += len;
                    len += 1;
                }
            }
        }
    }
    println!(
        "{}",
        sum.iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap()
            + 1
    );
}
