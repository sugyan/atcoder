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
    let (mut lo, mut hi) = (0, 100_000);
    while lo < hi {
        while lo < hi && v[lo] <= 1 {
            lo += 1;
        }
        while lo < hi && v[hi] <= 1 {
            hi -= 1;
        }
        if v[lo] > 1 || v[hi] > 1 {
            v[lo] -= 1;
            v[hi] -= 1;
        }
    }
    while v[lo] > 1 {
        v[lo] -= 2;
    }
    println!("{}", v.iter().sum::<i32>());
}
