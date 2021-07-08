use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        abc: [(usize, usize, u64); m],
    }
    let mut d = vec![vec![None; n]; n];
    for &(a, b, c) in &abc {
        d[a - 1][b - 1] = Some(c);
    }
    let mut answer = 0;
    for k in 0..n {
        for s in 0..n {
            for t in (0..n).filter(|&t| t != s) {
                if let (Some(sk), Some(kt)) = (d[s][k], d[k][t]) {
                    d[s][t] = Some(d[s][t].map_or(sk + kt, |c| c.min(sk + kt)));
                }
            }
            answer += d[s].iter().filter_map(|&o| o).sum::<u64>();
        }
    }
    println!("{}", answer);
}
