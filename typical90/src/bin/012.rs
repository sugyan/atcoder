use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        q: usize,
    }
    let mut grid = vec![vec![false; w]; h];
    let mut uf = UnionFind::new(h * w);
    for _ in 0..q {
        input! { t: u8 }
        if t == 1 {
            input! { r: usize, c: usize }
            grid[r - 1][c - 1] = true;
            for d in [0, 1, 0, !0, 0].windows(2) {
                let i = (r - 1).wrapping_add(d[0]);
                let j = (c - 1).wrapping_add(d[1]);
                if (0..h).contains(&i) && (0..w).contains(&j) && grid[i][j] {
                    uf.union(i * w + j, (r - 1) * w + c - 1);
                }
            }
        }
        if t == 2 {
            input! { ra: usize, ca: usize, rb: usize, cb: usize }
            let yes = grid[ra - 1][ca - 1]
                && grid[rb - 1][cb - 1]
                && uf.equiv((ra - 1) * w + ca - 1, (rb - 1) * w + cb - 1);
            println!("{}", if yes { "Yes" } else { "No" });
        }
    }
}
