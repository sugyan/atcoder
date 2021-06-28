use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        cp: [(usize, u32); n],
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut sums = vec![vec![0; n + 1]; 2];
    for (i, &(c, p)) in cp.iter().enumerate() {
        sums[c - 1][i + 1] = p;
        sums[0][i + 1] += sums[0][i];
        sums[1][i + 1] += sums[1][i];
    }
    for &(l, r) in &lr {
        println!(
            "{} {}",
            sums[0][r] - sums[0][l - 1],
            sums[1][r] - sums[1][l - 1]
        );
    }
}
