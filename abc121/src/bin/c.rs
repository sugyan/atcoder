use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, mut m: u64,
        mut ab: [(u64, u64); n],
    }
    ab.sort_unstable();
    let mut answer = 0;
    for &(a, b) in &ab {
        if b >= m {
            answer += a * m;
            break;
        }
        answer += a * b;
        m -= b;
    }
    println!("{}", answer);
}
