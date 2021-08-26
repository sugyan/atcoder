use proconio::marker::Chars;
use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let mut v = vec![vec![(0, 0, 0, 0); w]; h];
    v[0][0] = (1, 1, 1, 1);
    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                continue;
            }
            if s[i][j] == '.' {
                v[i][j].0 += if i > 0 { v[i - 1][j].1 } else { 0 };
                v[i][j].0 += if j > 0 { v[i][j - 1].2 } else { 0 };
                v[i][j].0 += if i > 0 && j > 0 { v[i - 1][j - 1].3 } else { 0 };
                v[i][j].1 += v[i][j].0;
                v[i][j].2 += v[i][j].0;
                v[i][j].3 += v[i][j].0;
                if i > 0 && s[i - 1][j] == '.' {
                    v[i][j].1 += v[i - 1][j].1;
                }
                if j > 0 && s[i][j - 1] == '.' {
                    v[i][j].2 += v[i][j - 1].2;
                }
                if i > 0 && j > 0 && s[i - 1][j - 1] == '.' {
                    v[i][j].3 += v[i - 1][j - 1].3;
                }
                v[i][j].0 %= MOD;
                v[i][j].1 %= MOD;
                v[i][j].2 %= MOD;
                v[i][j].3 %= MOD;
            }
        }
    }
    println!("{}", v[h - 1][w - 1].0);
}
