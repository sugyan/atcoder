use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut btm = BTreeMap::new();
    for &a in &a {
        *btm.entry(Reverse(a)).or_insert(0) += 1;
    }
    let v = btm
        .iter()
        .filter_map(|(&Reverse(k), &v)| if v > 1 { Some((k, v)) } else { None })
        .take(2)
        .collect::<Vec<_>>();
    let answer = if !v.is_empty() && v[0].1 >= 4 {
        v[0].0.pow(2)
    } else if v.len() > 1 {
        v[0].0 * v[1].0
    } else {
        0
    };
    println!("{}", answer);
}
