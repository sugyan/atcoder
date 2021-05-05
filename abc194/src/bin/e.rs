use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [u32; n],
    }
    let mut v = vec![None; n];
    let mut prev = vec![0; n];
    for (i, &a) in a.iter().enumerate() {
        let d = i - prev[a as usize] + 1;
        v[a as usize] = Some(v[a as usize].map_or(d, |v: usize| v.max(d)));
        prev[a as usize] = i + 1;
    }
    for i in 0..n {
        v[i] = v[i].map(|v| v.max(n + 1 - prev[i]));
    }
    println!(
        "{}",
        v.iter()
            .position(|&e| match e {
                Some(w) if w <= m => false,
                _ => true,
            })
            .unwrap_or(n)
    );
}
