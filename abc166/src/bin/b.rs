use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        a: [[usize]; k],
    }
    let mut v = vec![false; n];
    for a in &a {
        for &i in a {
            v[i - 1] = true;
        }
    }
    println!("{:?}", v.iter().filter(|&b| !b).count());
}
