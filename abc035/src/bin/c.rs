use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        lr: [(Usize1, usize); q],
    }
    let mut v = vec![false; n + 1];
    for &(l, r) in &lr {
        v[l] = !v[l];
        v[r] = !v[r];
    }
    let mut answer = String::new();
    let mut b = 0;
    (0..n).for_each(|i| {
        if v[i] {
            b = 1 - b;
        }
        answer.push((b + b'0') as char);
    });
    println!("{}", answer);
}
