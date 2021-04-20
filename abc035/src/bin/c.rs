use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        lr: [(usize, usize); q],
    }
    let mut v = vec![false; n + 1];
    for &(l, r) in &lr {
        v[l - 1] = !v[l - 1];
        v[r] = !v[r];
    }
    let answer = (0..n)
        .scan(0, |state, i| {
            if v[i] {
                *state = 1 - *state;
            }
            Some(*state)
        })
        .map(|b| (b + b'0') as char)
        .collect::<String>();
    println!("{}", answer);
}
