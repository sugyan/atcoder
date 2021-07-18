use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n + 1],
    }
    let mut sum = a.clone();
    for i in (0..n).rev() {
        sum[i] += sum[i + 1];
    }
    sum.push(0);
    let mut b = vec![0; n + 1];
    if a[0] > 1 {
        println!("-1");
        return;
    }
    b[0] = 1 - a[0];
    let mut answer = 1;
    for i in 1..=n {
        if a[i] > b[i - 1] * 2 {
            println!("-1");
            return;
        }
        b[i] = (2 * b[i - 1] - a[i]).min(sum[i + 1]);
        answer += a[i] + b[i];
    }
    println!("{}", answer);
}
