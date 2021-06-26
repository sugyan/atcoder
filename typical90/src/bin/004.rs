use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        a: [[u32; w]; h],
    }
    let row = (0..h)
        .map(|i| (0..w).map(|j| a[i][j]).sum())
        .collect::<Vec<u32>>();
    let col = (0..w)
        .map(|j| (0..h).map(|i| a[i][j]).sum())
        .collect::<Vec<u32>>();
    for i in 0..h {
        for j in 0..w {
            print!("{}", row[i] + col[j] - a[i][j]);
            if j < w - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
