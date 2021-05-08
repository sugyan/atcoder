use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        a: [u32; n],
    }
    let mut answer = 0;
    let mut len = 1;
    for i in 0..n - 1 {
        len = if a[i] < a[i + 1] { len + 1 } else { 1 };
        if len >= k {
            answer += 1;
        }
    }
    println!("{}", if k > 1 { answer } else { n });
}
