use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, x: i64,
        mut a: [i64; n],
    }
    let mut answer = 0;
    for i in 0..n - 1 {
        let d = a[i] + a[i + 1] - x;
        if d > 0 {
            answer += d;
            a[i + 1] = 0.max(a[i + 1] - d);
        }
    }
    println!("{:?}", answer);
}
