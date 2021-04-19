use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut answer = 0;
    let (mut sum, mut sumsum, mut max) = (0, 0, 0);
    for &a in &a {
        sum += a;
        max = max.max(sum);
        answer = answer.max(sumsum + max);
        sumsum += sum;
    }
    println!("{}", answer);
}
