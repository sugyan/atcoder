use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n],
    }
    let mut s = ab.iter().map(|&(a, b)| a / b).sum::<f64>() / 2.0;
    let mut answer = 0.0;
    for &(a, b) in &ab {
        if a / b > s {
            answer += b * s;
            break;
        }
        answer += a;
        s -= a / b;
    }
    println!("{}", answer);
}
