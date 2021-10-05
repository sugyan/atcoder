use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let mut answer = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                answer = answer.max(bfs(&s, (i, j), h, w));
            }
        }
    }
    println!("{}", answer)
}

fn bfs(s: &[Vec<char>], p: (usize, usize), h: usize, w: usize) -> i32 {
    let mut visited = vec![vec![false; w]; h];
    let mut vd = VecDeque::new();
    vd.push_back((p, 0));
    visited[p.0][p.1] = true;
    let mut ret = 0;
    while let Some((p, m)) = vd.pop_front() {
        ret = ret.max(m);
        for d in [0, 1, 0, !0, 0].windows(2) {
            let i = p.0.wrapping_add(d[0]);
            let j = p.1.wrapping_add(d[1]);
            if (0..h).contains(&i) && (0..w).contains(&j) && s[i][j] == '.' && !visited[i][j] {
                visited[i][j] = true;
                vd.push_back(((i, j), m + 1));
            }
        }
    }
    ret
}
