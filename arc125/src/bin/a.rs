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
    } else if let Some(l) = s.iter().position(|&u| u != s[0]) {
        (l.min(n - s.iter().rposition(|&u| u != s[0]).unwrap()) + m
            - if s[0] != t[0] { 0 } else { 1 }
            + t.windows(2).filter(|w| w[0] != w[1]).count()) as i32
    } else {
        -1
    };
    println!("{}", answer);
}
