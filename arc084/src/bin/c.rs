use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        mut b: [i32; n],
        mut c: [i32; n],
    }
    a.sort_unstable();
    b.sort_unstable();
    c.sort_unstable();
    let mut answer = 0;
    let (mut i, mut k) = (0, 0);
    for &b in &b {
        while i < n && a[i] < b {
            i += 1;
        }
        while k < n && c[k] <= b {
            k += 1;
        }
        answer += i * (n - k);
    }
    println!("{}", answer);
}
