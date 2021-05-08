use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        a: [u32; n],
    }
    let mut answer = 0;
    if k > 1 {
        let mut len = 0;
        for i in 0..n - 1 {
            if a[i] < a[i + 1] {
                len += 1;
                if len >= k - 1 {
                    answer += 1;
                }
            } else {
                len = 0;
            }
        }
    } else {
        answer = n;
    }
    println!("{}", answer);
}
