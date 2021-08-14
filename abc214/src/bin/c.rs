use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [u32; n],
        t: [u32; n],
    }
    let mut v = t.clone();
    if let Some(min) = (0..n).min_by_key(|&i| t[i]) {
        for i in 1..n {
            let j = (min + i) % n;
            v[j] = v[j].min(v[(j + n - 1) % n] + s[(j + n - 1) % n]);
        }
    }
    for a in v {
        println!("{}", a);
    }
}
