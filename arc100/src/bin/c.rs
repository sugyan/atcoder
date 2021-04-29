use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut btm = BTreeMap::new();
    let mut sum = 0;
    for (i, &a) in a.iter().enumerate() {
        let s = a - i as i64 + 1;
        *btm.entry(s).or_insert(0) += 1;
        sum += s;
    }
    let mut answer = std::i64::MAX;
    let (mut i, mut m) = (0, 0);
    for (k, v) in &btm {
        sum -= (n - i) as i64 * (k - m);
        sum += i as i64 * (k - m);
        answer = answer.min(sum);
        m = *k;
        i += v;
    }
    println!("{}", answer);
}
