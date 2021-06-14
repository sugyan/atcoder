use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        sc: [(usize, u32); m],
    }
    let mut v = vec![None; n];
    for &(s, c) in &sc {
        if v[s - 1].map(|p| p != c).unwrap_or(false) || (n > 1 && s == 1 && c == 0) {
            println!("-1");
            return;
        }
        v[s - 1] = Some(c);
    }
    let answer = v.iter().enumerate().fold(0, |acc, (i, x)| {
        acc * 10 + x.unwrap_or(if i == 0 { 1 } else { 0 })
    });
    println!("{}", if n == 1 && m == 0 { 0 } else { answer });
}
