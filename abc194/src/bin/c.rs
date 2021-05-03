use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut hm = HashMap::new();
    for &a in &a {
        *hm.entry(a).or_default() += 1;
    }
    let keys = hm.keys().collect::<Vec<_>>();
    let mut answer = 0;
    for i in 1..keys.len() {
        for j in 0..i {
            if let (Some(vi), Some(vj)) = (hm.get(&keys[i]), hm.get(&keys[j])) {
                answer += ((keys[i] - keys[j]) * (keys[i] - keys[j])) as u64 * vi * vj;
            }
        }
    }
    println!("{}", answer);
}
