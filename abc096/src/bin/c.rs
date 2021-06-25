use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let mut ok = true;
    for i in 0..h {
        for j in 0..w {
            let adjacents = |d: &&[usize]| {
                let (i, j) = (i.wrapping_add(d[0]), j.wrapping_add(d[1]));
                (0..h).contains(&i) && (0..w).contains(&j) && s[i][j] == '#'
            };
            if s[i][j] == '#' && [0, 1, 0, !0, 0].windows(2).filter(adjacents).count() == 0 {
                ok = false;
            }
        }
    }
    println!("{}", if ok { "Yes" } else { "No" })
}
