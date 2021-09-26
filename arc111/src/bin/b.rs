use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(u32, u32); n],
    }
    let mut hm = HashMap::new();
    for &(a, b) in &ab {
        hm.entry(a).or_insert_with(HashSet::new).insert(b);
        hm.entry(b).or_insert_with(HashSet::new).insert(a);
    }
    let mut v = Vec::new();
    for (&k, hs) in &hm {
        if hs.len() == 1 {
            v.push(k);
        }
    }
    let mut answer = 0;
    while let Some(last) = v.pop() {
        let hs = hm.get_mut(&last).unwrap();
        if hs.len() != 1 {
            continue;
        }
        answer += 1;
        let i = hs.drain().next().unwrap();
        if let Some(hs) = hm.get_mut(&i) {
            hs.remove(&last);
            if hs.len() == 1 {
                v.push(i)
            }
        }
    }
    answer += hm.values().filter(|hs| !hs.is_empty()).count();
    println!("{}", answer);
}
