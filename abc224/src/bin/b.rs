use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        a: [[u32; w]; h],
    }
    for i1 in 0..h - 1 {
        for i2 in i1 + 1..h {
            for j1 in 0..w - 1 {
                for j2 in j1..w {
                    if a[i1][j1] + a[i2][j2] > a[i1][j2] + a[i2][j1] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    println!("Yes");
}
