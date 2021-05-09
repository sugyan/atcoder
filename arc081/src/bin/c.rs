use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut hm = HashMap::<u64, usize>::new();
    for &a in &a {
        *hm.entry(a).or_default() += 1;
    }
    let mut v = hm
        .iter()
        .filter_map(|(&k, &v)| if v > 1 { Some((k, v)) } else { None })
        .collect::<Vec<_>>();
    v.sort_unstable();
    v.reverse();
    if !v.is_empty() && v[0].1 >= 4 {
        println!("{}", v[0].0.pow(2));
    } else if v.len() > 1 {
        println!("{}", v[0].0 * v[1].0);
    } else {
        println!(0);
    }
}
