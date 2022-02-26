use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! { q: usize }
    let mut btm = BTreeMap::new();
    for _ in 0..q {
        input! { command: u8 }
        if command == 1 {
            input! { x: u64 }
            *btm.entry(x).or_insert(0) += 1;
        } else {
            input! { x: u64, k: u64 }
            if command == 2 {
                let mut sum = 0;
                for e in btm.range(..=x).rev() {
                    sum += e.1;
                    if sum >= k {
                        println!("{}", e.0);
                        break;
                    }
                }
                if sum < k {
                    println!("-1");
                }
            }
            if command == 3 {
                let mut sum = 0;
                for e in btm.range(x..) {
                    sum += e.1;
                    if sum >= k {
                        println!("{}", e.0);
                        break;
                    }
                }
                if sum < k {
                    println!("-1");
                }
            }
        }
    }
}
