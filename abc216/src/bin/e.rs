use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, mut k: i64,
        mut a: [i64; n],
    }
    a.sort();
    let mut p = a[n - 1];
    for (i, j) in (0..n).zip(1..) {
        p = if i < n - 1 { a[n - i - 2] } else { 0 };
        if (a[n - i - 1] - p) * j > k {
            p = a[n - i - 1] - k / j;
            k %= j;
            break;
        } else {
            k -= (a[n - i - 1] - p) * j;
        }
    }
    let mut answer = p * k;
    for &a in a.iter().filter(|&a| *a > p) {
        answer += (a * (a + 1) - p * (p + 1)) / 2;
    }
    println!("{}", answer);
}
