use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    let mut idx = (0..n).collect::<Vec<_>>();
    let mut v = Vec::<usize>::new();
    idx.sort_by_cached_key(|&i| xy[i].0);
    v.extend(&[idx[0], idx[1], idx[n - 1], idx[n - 2]]);
    idx.sort_by_cached_key(|&i| xy[i].1);
    v.extend(&[idx[0], idx[1], idx[n - 1], idx[n - 2]]);
    v.sort_unstable();
    v.dedup();
    let mut d = v
        .iter()
        .combinations(2)
        .map(|v| {
            let (&i, &j) = (v[0], v[1]);
            (xy[i].0 - xy[j].0).abs().max((xy[i].1 - xy[j].1).abs())
        })
        .collect::<Vec<_>>();
    d.sort_unstable();
    println!("{}", d[d.len() - 2]);
}
