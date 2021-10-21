use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut hm = HashMap::new();
    let mut answer = 0_u64;
    for (i, &a) in a.iter().enumerate() {
        *hm.entry(i as i64 + a).or_insert(0) += 1;
        answer += hm.get(&(i as i64 - a)).unwrap_or(&0);
    }
    println!("{}", answer);
}
