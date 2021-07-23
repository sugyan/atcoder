use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = vec![0; 100_001];
    for &a in &a {
        v[a] += 1;
    }
    let k = v.iter().filter(|&m| *m > 0).count();
    println!("{}", k + k % 2 - 1);
}
