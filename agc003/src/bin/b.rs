use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    let mut answer = 0;
    for i in 0..n {
        answer += a[i] / 2;
        if a[i] & 1 > 0 && i < n - 1 && a[i + 1] > 0 {
            answer += 1;
            a[i + 1] -= 1;
        }
    }
    println!("{}", answer);
}
