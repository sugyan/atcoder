use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: usize, c: usize, d: usize,
        a: [[u32; c]; r],
    }
    let mut answer = 0;
    for (i, row) in a.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if i + j <= d && (i + j) % 2 == d % 2 {
                answer = answer.max(col);
            }
        }
    }
    println!("{}", answer);
}
