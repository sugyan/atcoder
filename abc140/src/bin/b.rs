use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [u32; n],
        c: [u32; n - 1],
    }
    let mut answer = 0;
    for i in 0..n {
        answer += b[a[i] - 1];
        if i < n - 1 && a[i] + 1 == a[i + 1] {
            answer += c[a[i] - 1];
        }
    }
    println!("{}", answer);
}
