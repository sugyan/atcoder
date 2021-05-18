use proconio::marker::Bytes;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: Bytes,
    }
    let keys = [b'A', b'B', b'X', b'Y'];
    let mut shortcuts = Vec::new();
    for &k1 in &keys {
        for &k2 in &keys {
            shortcuts.push((k1, k2));
        }
    }
    let mut answer = n;
    for &s1 in &shortcuts {
        for &s2 in &shortcuts {
            if s1 == s2 {
                continue;
            }
            let mut count = 0;
            let mut i = 0;
            while i < n {
                if i < n - 1 && ((c[i], c[i + 1]) == s1 || (c[i], c[i + 1]) == s2) {
                    i += 1;
                }
                count += 1;
                i += 1;
            }
            answer = answer.min(count);
        }
    }
    println!("{}", answer);
}
