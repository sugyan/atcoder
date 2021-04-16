use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [u32; n],
    }
    let mut v = vec![1; n];
    let mut l = 0;
    for i in 1..n {
        l = if h[i] > h[i - 1] { l + 1 } else { 0 };
        v[i] += l;
    }
    for i in (0..n - 1).rev() {
        l = if h[i] > h[i + 1] { l + 1 } else { 0 };
        v[i] += l;
    }
    println!("{}", v.iter().max().unwrap());
}
