use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        n: usize, c: i64,
        abc: [(usize, usize, i64); n],
    }
    let mut btm = BTreeMap::new();
    for &(a, b, c) in &abc {
        *btm.entry(a).or_insert(0) += c;
        *btm.entry(b + 1).or_insert(0) -= c;
    }
    let mut sum = 0;
    let (mut prev, mut cost) = (None, 0);
    for (&k, &v) in &btm {
        if let Some(p) = prev {
            sum += cost.min(c) * (k - p) as i64;
        }
        cost += v;
        prev = Some(k);
    }
    println!("{}", sum);
}
