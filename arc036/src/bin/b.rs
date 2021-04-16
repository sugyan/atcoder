use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [u32; n],
    }
    let mut v = vec![0; n];
    let mut l = 0;
    for i in 1..n {
        if h[i] > h[i - 1] {
            l += 1;
        } else {
            l = 0;
        }
        v[i] += l;
    }
    for i in (0..n - 1).rev() {
        if h[i] > h[i + 1] {
            l += 1;
        } else {
            l = 0;
        }
        v[i] += l;
    }
    println!("{}", v.iter().max().unwrap() + 1);
}
