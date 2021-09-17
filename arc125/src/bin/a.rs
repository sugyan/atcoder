use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        s: [u8; n],
        t: [u8; m],
    }
    let answer = if t.iter().all(|&u| u == s[0]) {
        m as i32
    } else {
        match (
            s.iter().position(|&u| u != s[0]),
            s.iter().rposition(|&u| u != s[0]),
        ) {
            (Some(l), Some(r)) => {
                (l.min(n - r)
                    + m
                    + if s[0] != t[0] { 1 } else { 0 }
                    + t.windows(2).filter(|w| w[0] != w[1]).count()
                    - 1) as i32
            }
            _ => -1,
        }
    };
    println!("{}", answer);
}
