use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let mut answer = 0;
    for i in 1..h {
        for j in 1..w {
            let d = [s[i - 1][j - 1], s[i - 1][j], s[i][j - 1], s[i][j]]
                .iter()
                .fold((0, 0), |a, &c| {
                    if c == '#' {
                        (a.0 + 1, a.1)
                    } else {
                        (a.0, a.1 + 1)
                    }
                });
            if d == (1, 3) || d == (3, 1) {
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}
