use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }
    let s = s.as_bytes().iter().map(|u| u - b'0').collect::<Vec<_>>();
    let mut answer = 0;
    for i in 0..=9 {
        let pi = s.iter().position(|&u| u == i);
        for j in 0..=9 {
            let pj = s.iter().rposition(|&u| u == j);
            match (pi, pj) {
                (Some(lo), Some(hi)) if lo < hi => {
                    let mut d = [false; 10];
                    for k in lo + 1..hi {
                        d[s[k] as usize] = true;
                    }
                    answer += d.iter().filter(|&b| *b).count();
                }
                _ => {}
            }
        }
    }
    println!("{}", answer);
}
