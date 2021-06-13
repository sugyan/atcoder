use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        c: (usize, usize),
        d: (usize, usize),
        s: [Chars; h],
    }
    let mut mins = vec![vec![None; w]; h];
    let mut vd = VecDeque::new();
    vd.push_back(((c.0 - 1, c.1 - 1), 0));
    while let Some((p, d)) = vd.pop_front() {
        if mins[p.0][p.1].is_some() {
            continue;
        }
        let hs = bfs(&s, p);
        for &(i, j) in &hs {
            mins[i][j] = Some(d);
        }
        for &(i, j) in &hs {
            for di in -2..=2 {
                for dj in -2..=2 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let (i, j) = (i as i32 + di, j as i32 + dj);
                    if (0..h as i32).contains(&i) && (0..w as i32).contains(&j) {
                        let (i, j) = (i as usize, j as usize);
                        if s[i][j] == '.' && mins[i][j].is_none() {
                            vd.push_back(((i, j), d + 1));
                        }
                    }
                }
            }
        }
    }
    if let Some(d) = mins[d.0 - 1][d.1 - 1] {
        println!("{}", d);
    } else {
        println!("-1");
    }
}

fn bfs(s: &[Vec<char>], p: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut ret = HashSet::new();
    let mut vd = VecDeque::new();
    vd.push_back(p);
    while let Some((i, j)) = vd.pop_front() {
        if ret.contains(&(i, j)) {
            continue;
        }
        ret.insert((i, j));
        if i > 0 && s[i - 1][j] == '.' {
            vd.push_back((i - 1, j));
        }
        if j > 0 && s[i][j - 1] == '.' {
            vd.push_back((i, j - 1));
        }
        if i < s.len() - 1 && s[i + 1][j] == '.' {
            vd.push_back((i + 1, j));
        }
        if j < s[0].len() - 1 && s[i][j + 1] == '.' {
            vd.push_back((i, j + 1));
        }
    }
    ret
}
