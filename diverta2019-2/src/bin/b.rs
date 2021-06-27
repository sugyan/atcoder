use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    let mut hm = HashMap::new();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                *hm.entry((xy[i].0 - xy[j].0, xy[i].1 - xy[j].1))
                    .or_insert(0) += 1;
            }
        }
    }
    let mut answer = n;
    if let Some(&max) = hm.values().max() {
        answer -= max;
    }
    println!("{}", answer);
}
