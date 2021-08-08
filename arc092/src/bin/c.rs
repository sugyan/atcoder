use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
        mut cd: [(usize, usize); n],
    }
    ab.sort_by_cached_key(|&(_, y)| 2 * n - y);
    cd.sort_by_cached_key(|&(x, _)| x);
    let mut used = vec![false; n];
    let mut answer = 0;
    for &(c, d) in &cd {
        if let Some(i) = (0..n).position(|i| !used[i] && ab[i].0 < c && ab[i].1 < d) {
            answer += 1;
            used[i] = true;
        }
    }
    println!("{}", answer);
}
