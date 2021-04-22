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
    let mut bc = vec![0; n];
    let (mut j, mut k) = (0, 0);
    for (j, &b) in b.iter().enumerate() {
        while k < n && c[k] <= b {
            k += 1;
        }
        bc[j] += n - k;
    }
    for i in (0..n - 1).rev() {
        bc[i] += bc[i + 1];
    }
    let mut answer = 0;
    for &a in &a {
        while j < n && b[j] <= a {
            j += 1;
        }
        if j < n {
            answer += bc[j];
        }
    }
    println!("{}", answer);
}
