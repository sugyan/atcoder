use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, mr: (usize, usize),
        r: [usize; mr.1],
        abc: [(usize, usize, u32); mr.0],
    }
    let mut d = vec![vec![std::u32::MAX; n]; n];
    for &(a, b, c) in &abc {
        d[a - 1][b - 1] = c;
        d[b - 1][a - 1] = c;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k].saturating_add(d[k][j]));
            }
        }
    }
    let answer = r
        .iter()
        .permutations(mr.1)
        .map(|p| p.windows(2).map(|w| d[w[0] - 1][w[1] - 1]).sum::<u32>())
        .min()
        .unwrap();
    println!("{}", answer);
}
