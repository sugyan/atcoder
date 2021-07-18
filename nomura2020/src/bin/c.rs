use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n + 1],
    }
    let mut sum = a.iter().sum::<u64>();
    let mut answer = 1;
    let mut b = 1;
    for &a in &a {
        if a > b {
            println!("-1");
            return;
        }
        sum -= a;
        b = ((b - a) * 2).min(sum);
        answer += b;
    }
    println!("{}", answer);
}
