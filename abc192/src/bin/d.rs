use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: String,
        m: u64,
    }
    let x = x
        .as_bytes()
        .iter()
        .map(|&b| (b - b'0') as u64)
        .collect::<Vec<_>>();
    if x.len() == 1 {
        println!("{}", if x[0] > m { 0 } else { 1 });
        return;
    }
    let d = x.iter().max().unwrap();
    let (mut lo, mut hi) = (d + 1, m + 1);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if m < x
            .iter()
            .fold(0, |acc, &x| acc.saturating_mul(mid).saturating_add(x))
        {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    println!("{}", lo - d - 1);
}
