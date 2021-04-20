use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, n: usize, m: usize,
        ab: [(usize, usize); n],
        cd: [(usize, usize); m],
    }
    let mut grid = vec![vec![0_u8; w]; h];
    for &(i, j) in &ab {
        grid[i - 1][j - 1] = 2;
    }
    for &(i, j) in &cd {
        grid[i - 1][j - 1] = 2;
    }
    let mut answer = n;
    for &(i, j) in &ab {
        for d in [0, !0, 0, 1, 0].windows(2) {
            let mut i = (i - 1).wrapping_add(d[0]);
            let mut j = (j - 1).wrapping_add(d[1]);
            while i < h && j < w && grid[i][j] < 2 {
                if grid[i][j] == 0 {
                    answer += 1;
                    grid[i][j] = 1;
                }
                i = i.wrapping_add(d[0]);
                j = j.wrapping_add(d[1]);
            }
        }
    }
    println!("{}", answer);
}
