use proconio::marker::Bytes;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Bytes; h],
    }
    let mut c = vec![vec![b'0'; w]; h];
    for (i, row) in s.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col != b'#' {
                continue;
            }
            for &di in &[!0, 0, 1] {
                let i = i.wrapping_add(di);
                for &dj in &[!0, 0, 1] {
                    let j = j.wrapping_add(dj);
                    if i < h && j < w {
                        c[i][j] += 1;
                    }
                }
            }
        }
    }
    for (i, row) in s.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            print!("{}", if *col == b'#' { b'#' } else { c[i][j] } as char)
        }
        println!();
    }
}
