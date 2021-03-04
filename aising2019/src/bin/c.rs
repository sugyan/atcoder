use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    let mut answer = 0_u64;
    let mut visited = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            let (mut black, mut white) = (0, 0);
            let mut vd = VecDeque::new();
            vd.push_back((i, j));
            while let Some((i, j)) = vd.pop_front() {
                match s[i][j] {
                    '#' => black += 1,
                    '.' => white += 1,
                    _ => {}
                }
                for &(di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                    let (ni, nj) = (i as i32 + di, j as i32 + dj);
                    if !(0..h as i32).contains(&ni) || !(0..w as i32).contains(&nj) {
                        continue;
                    }
                    let (ni, nj) = (ni as usize, nj as usize);
                    if !visited[ni][nj] && s[ni][nj] != s[i][j] {
                        visited[ni][nj] = true;
                        vd.push_back((ni, nj));
                    }
                }
            }
            answer += black * white;
        }
    }
    println!("{}", answer);
}
