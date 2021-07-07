use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    let mut x = xy
        .iter()
        .enumerate()
        .map(|(i, &(x, _))| (x, i))
        .collect::<Vec<_>>();
    let mut y = xy
        .iter()
        .enumerate()
        .map(|(i, &(_, y))| (y, i))
        .collect::<Vec<_>>();
    x.sort();
    y.sort();
    let mut v = vec![
        (x[0].1, x[n - 1].1),
        (x[1].1, x[n - 1].1),
        (x[0].1, x[n - 2].1),
        (y[0].1, y[n - 1].1),
        (y[1].1, y[n - 1].1),
        (y[0].1, y[n - 2].1),
    ];
    v.sort();
    v.dedup();
    let mut res = v
        .iter()
        .map(|&(i, j)| (xy[i].0 - xy[j].0).abs().max((xy[i].1 - xy[j].1).abs()))
        .collect::<Vec<_>>();
    res.sort_unstable();
    println!("{}", res[res.len() - 2]);
}
