use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        c: (usize, usize),
        d: (usize, usize),
        s: [Chars; h],
    }
    let mut mins = vec![vec![std::usize::MAX; w]; h];
    let mut vd = VecDeque::new();
    vd.push_back(((c.0 - 1, c.1 - 1), 0));
    while let Some(((i, j), l)) = vd.pop_front() {
        if i == d.0 - 1 && j == d.1 - 1 {
            println!("{}", l);
            return;
        }
        for d in [0, 1, 0, -1, 0].windows(2) {
            let i = i as isize + d[0];
            let j = j as isize + d[1];
            if (0..h as isize).contains(&i) && (0..w as isize).contains(&j) {
                let (i, j) = (i as usize, j as usize);
                if s[i][j] == '.' && mins[i][j] > l {
                    mins[i][j] = l;
                    vd.push_front(((i, j), l));
                }
            }
        }
        for di in -2..=2 {
            for dj in -2..=2 {
                let i = i as isize + di;
                let j = j as isize + dj;
                if (0..h as isize).contains(&i) && (0..w as isize).contains(&j) {
                    let (i, j) = (i as usize, j as usize);
                    if i < h && j < w && s[i][j] == '.' && mins[i][j] > l + 1 {
                        mins[i][j] = l + 1;
                        vd.push_back(((i, j), l + 1));
                    }
                }
            }
        }
    }
    println!("-1");
}
