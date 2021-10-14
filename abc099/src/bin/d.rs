use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        nc: (usize, usize),
        d: [[u32; nc.1]; nc.1],
        c: [[usize; nc.0]; nc.0],
    }
    let mut costs = vec![vec![0; nc.1]; 3];
    for i in 0..nc.0 {
        for j in 0..nc.0 {
            for k in 0..nc.1 {
                costs[(i + j) % 3][k] += d[c[i][j] - 1][k];
            }
        }
    }
    let answer = (0..nc.1)
        .permutations(3)
        .map(|colors| {
            colors
                .iter()
                .enumerate()
                .map(|(i, &c)| costs[i][c])
                .sum::<u32>()
        })
        .min()
        .unwrap();
    println!("{}", answer);
}
