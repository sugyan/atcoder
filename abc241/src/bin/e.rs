use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }
    let mut hm = HashMap::new();
    let mut answer = 0;
    let mut i = 0;
    while i < k {
        answer += a[answer % n];
        if let Some(&(j, prev)) = hm.get(&(answer % n)) {
            let m = (k - 1 - i) / (i - j);
            answer += m * (answer - prev);
            i += m * (i - j);
        } else {
            hm.insert(answer % n, (i, answer));
        }
        i += 1;
    }
    println!("{}", answer);
}
