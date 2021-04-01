use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize, w: usize,
        a: [Chars; h],
    }
    let possible = || -> bool {
        for i in 0..h {
            for j in 0..w {
                if a[i][j] == '#' {
                    if i > 0 && j > 0 && a[i - 1][j] == '#' && a[i][j - 1] == '#' {
                        return false;
                    }
                    if i < h - 1 && j < w - 1 && a[i + 1][j] == '#' && a[i][j + 1] == '#' {
                        return false;
                    }
                }
            }
        }
        true
    };
    println!("{}", if possible() { "Possible" } else { "Impossible" });
}
