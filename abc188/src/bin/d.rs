use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, c: i64,
        abc: [(i64, i64, i64); n],
    }
    let mut v = Vec::with_capacity(n * 2);
    for &(a, b, c) in &abc {
        v.push((a, c));
        v.push((b + 1, -c));
    }
    v.sort();
    let mut sum = 0;
    let (mut prev, mut cost) = (0, 0);
    for &(i, m) in &v {
        sum += cost.min(c) * (i - prev);
        cost += m;
        prev = i;
    }
    println!("{}", sum);
}
