use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
    }
    let mut v = vec![0; n * 3];
    for i in 1..n * 2 {
        v[i] = if i <= n { v[i - 1] + 1 } else { v[i - 1] - 1 }
    }
    let mut sum = 0;
    let mut nums = vec![0; n * 3];
    for i in 0..n * 3 {
        sum += v[i];
        if i >= n {
            sum -= v[i - n];
        }
        nums[i] = sum;
    }
    sum = 0;
    let mut x = 0;
    while x < n * 3 && sum + nums[x] < k {
        sum += nums[x];
        x += 1;
    }
    let mut i = 1;
    while sum + v[x - i + 1] < k {
        sum += v[x - i + 1];
        i += 1;
    }
    let remain = x + 2 - i;
    let j = k - sum + (remain - 1).saturating_sub(n);
    println!("{} {} {}", i, j, remain - j);
}
