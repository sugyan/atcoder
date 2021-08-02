use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut v = vec![0; n + 1];
    for &p in &p {
        v[p] = v[p - 1] + 1;
    }
    println!("{}", n - v.iter().max().unwrap());
}
