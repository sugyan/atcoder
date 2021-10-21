use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut hm = HashMap::new();
    for (i, &a) in a.iter().enumerate() {
        *hm.entry((n - i) as i64 - a).or_insert(0_u64) += 1;
    }
    let mut answer = 0;
    for (i, &a) in a.iter().rev().enumerate() {
        answer += hm.get(&(a + i as i64 + 1)).unwrap_or(&0);
    }
    println!("{}", answer);
}
