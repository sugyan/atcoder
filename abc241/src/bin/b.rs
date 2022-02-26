use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [u32; n],
        b: [u32; m],
    }
    let mut map = HashMap::new();
    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }
    for b in b {
        if let Some(c) = map.get_mut(&b) {
            *c -= 1;
            if *c == 0 {
                map.remove(&b);
            }
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
