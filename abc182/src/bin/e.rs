use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, n: usize, m: usize,
        ab: [(usize, usize); n],
        cd: [(usize, usize); m],
    }
    let mut grid = vec![vec![None; w]; h];
    for &(i, j) in &ab {
        grid[i - 1][j - 1] = Some(true);
    }
    for &(i, j) in &cd {
        grid[i - 1][j - 1] = Some(false);
    }
    let mut reachable = vec![vec![false; w]; h];
    for i in 0..h {
        let mut reach = false;
        for j in 0..w {
            if let Some(b) = grid[i][j] {
                reach = b;
            }
            if reach {
                reachable[i][j] = true;
            }
        }
        for j in (0..w).rev() {
            if let Some(b) = grid[i][j] {
                reach = b;
            }
            if reach {
                reachable[i][j] = true;
            }
        }
    }
    for j in 0..w {
        let mut reach = false;
        for i in 0..h {
            if let Some(b) = grid[i][j] {
                reach = b;
            }
            if reach {
                reachable[i][j] = true;
            }
        }
        for i in (0..h).rev() {
            if let Some(b) = grid[i][j] {
                reach = b;
            }
            if reach {
                reachable[i][j] = true;
            }
        }
    }
    let answer = reachable
        .iter()
        .map(|row| row.iter().filter(|&b| *b).count())
        .sum::<usize>();
    println!("{}", answer);
}
