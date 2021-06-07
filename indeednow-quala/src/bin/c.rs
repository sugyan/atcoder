use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [u32; n],
        q: usize,
        k: [u32; q],
    }
    let mut btm = BTreeMap::new();
    for &s in &s {
        *btm.entry(s).or_insert(0) += 1;
    }
    let mut v = Vec::new();
    let mut sum = 0;
    for (&key, &val) in btm.iter().rev() {
        v.push((sum, key + 1));
        sum += val;
    }
    v.push((sum, 1));
    for &k in &k {
        let answer = match v.binary_search_by_key(&k, |&(sum, _)| sum) {
            Ok(i) => v[i].1,
            Err(i) => v[i - 1].1,
        };
        println!("{}", if answer == 1 { 0 } else { answer });
    }
}
