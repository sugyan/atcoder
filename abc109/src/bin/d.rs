use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        mut a: [[u8; w]; h],
    }
    let mut answers = Vec::new();
    for i in 0..h {
        let odds = (0..w).filter(|&j| a[i][j] & 1 == 1).collect::<Vec<_>>();
        for j in 0..odds.len() / 2 {
            for k in odds[j * 2]..odds[j * 2 + 1] {
                answers.push((i, k, i, k + 1));
            }
        }
        if i < h - 1 && odds.len() & 1 == 1 {
            if let Some(&j) = odds.last() {
                answers.push((i, j, i + 1, j));
                a[i + 1][j] += 1;
            }
        }
    }
    println!("{}", answers.len());
    for &a in &answers {
        println!("{} {} {} {}", a.0 + 1, a.1 + 1, a.2 + 1, a.3 + 1);
    }
}
