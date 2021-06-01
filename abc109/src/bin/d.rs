use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        mut a: [[u8; w]; h],
    }
    let mut answers = Vec::new();
    for i in 0..h {
        for j in 0..w - 1 {
            if a[i][j] & 1 == 1 {
                answers.push((i, j, i, j + 1));
                a[i][j + 1] += 1;
            }
        }
    }
    for i in 0..h - 1 {
        if a[i][w - 1] & 1 == 1 {
            answers.push((i, w - 1, i + 1, w - 1));
            a[i + 1][w - 1] += 1;
        }
    }
    println!("{}", answers.len());
    for &a in &answers {
        println!("{} {} {} {}", a.0 + 1, a.1 + 1, a.2 + 1, a.3 + 1);
    }
}
