use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [[u8; 3]; 3],
        n: usize,
        b: [u8; n],
    }
    let mut answer = false;
    for i in 0..3 {
        answer |= (0..3).all(|j| b.contains(&a[i][j]));
        answer |= (0..3).all(|j| b.contains(&a[j][i]));
    }
    answer |= (0..3).all(|i| b.contains(&a[i][i]));
    answer |= (0..3).all(|i| b.contains(&a[i][2 - i]));
    println!("{}", if answer { "Yes" } else { "No" });
}
