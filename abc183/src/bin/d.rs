use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, w: i64,
        stp: [(usize, usize, i64); n],
    }
    let mut v = vec![0; 200_001];
    for &(s, t, p) in &stp {
        v[s] += p;
        v[t] -= p;
    }
    (1..200_001).for_each(|i| v[i] += v[i - 1]);
    let yes = *v.iter().max().unwrap() <= w;
    println!("{}", if yes { "Yes" } else { "No" });
}
