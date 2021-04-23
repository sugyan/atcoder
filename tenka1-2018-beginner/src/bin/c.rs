use proconio::{fastout, input};
use std::iter::{empty, once, repeat};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort_unstable();
    let mut v = Vec::new();
    if n % 2 == 0 {
        v.push(
            empty()
                .chain(repeat(-2).take(n / 2 - 1))
                .chain(once(-1))
                .chain(once(1))
                .chain(repeat(2).take(n / 2 - 1)),
        );
    } else {
        v.push(
            empty()
                .chain(repeat(-2).take(n / 2))
                .chain(once(1))
                .chain(once(1))
                .chain(repeat(2).take(n / 2)),
        );
        v.push(
            empty()
                .chain(repeat(-2).take(n / 2 - 1))
                .chain(once(-1))
                .chain(once(-1))
                .chain(repeat(2).take(n / 2 + 1)),
        );
    }
    let mut answer = 0;
    for iter in v {
        answer = answer.max(a.iter().zip(iter).map(|(a, m)| a * m).sum())
    }
    println!("{}", answer);
}
