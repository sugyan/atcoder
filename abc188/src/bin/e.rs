use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [i32; n],
        mut xy: [(usize, usize); m],

    }
    let mut dp: Vec<Option<i32>> = vec![None; n];
    xy.sort();
    for &(x, y) in &xy {
        dp[y - 1] = Some(match (dp[x - 1], dp[y - 1]) {
            (Some(px), Some(py)) => py.max(px.max(0) + a[y - 1] - a[x - 1]),
            (Some(px), None) => px.max(0) + a[y - 1] - a[x - 1],
            (None, Some(py)) => py.max(a[y - 1] - a[x - 1]),
            (None, None) => a[y - 1] - a[x - 1],
        });
    }
    let answer = dp.iter().filter_map(|&v| v).max().unwrap();
    println!("{}", answer);
}
