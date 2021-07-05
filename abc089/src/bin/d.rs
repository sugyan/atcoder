use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, d: usize,
        a: [[usize; w]; h],
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut m = vec![(0, 0); h * w];
    for i in 0..h {
        for j in 0..w {
            m[a[i][j] - 1] = (i as isize, j as isize);
        }
    }
    let mut v = vec![0; h * w];
    for i in 0..h * w - d {
        let (p, q) = (m[i], m[i + d]);
        v[i + d] = v[i] + (p.0 - q.0).abs() + (p.1 - q.1).abs();
    }
    for &(l, r) in &lr {
        println!("{}", v[r - 1] - v[l - 1]);
    }
}
