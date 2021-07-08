use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
        m: usize,
        xy: [(usize, usize); m],
    }
    let mut ok = vec![vec![true; n]; n];
    for &(x, y) in &xy {
        ok[x - 1][y - 1] = false;
        ok[y - 1][x - 1] = false;
    }
    let mut answer = None;
    for p in (0..n).permutations(n) {
        if p.windows(2).all(|v| ok[v[0]][v[1]]) {
            let sum = p.iter().enumerate().map(|(i, &j)| a[j][i]).sum::<i32>();
            answer = Some(answer.map_or(sum, |min: i32| min.min(sum)));
        }
    }
    println!("{}", if let Some(a) = answer { a } else { -1 });
}
