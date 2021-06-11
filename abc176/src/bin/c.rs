use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let (mut answer, mut p) = (0, 0);
    for &a in &a {
        p = p.max(a);
        answer += p - a;
    }
    println!("{}", answer);
}
