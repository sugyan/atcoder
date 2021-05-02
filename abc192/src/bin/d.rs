use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: String,
        m: u128,
    }
    let x = x
        .as_bytes()
        .iter()
        .map(|&b| (b - b'0') as u128)
        .collect::<Vec<_>>();
    if x.len() == 1 {
        println!("{}", if x[0] > m { 0 } else { 1 });
        return;
    }
    let d = x.iter().max().unwrap();
    let (mut lo, mut hi) = (d + 1, m + 1);
    let is_valid = |n: u128| -> bool {
        let mut val = 0;
        for &u in &x {
            val = val * n + u;
            if val > m {
                return false;
            }
        }
        true
    };
    while lo < hi {
        let mid = (lo + hi) / 2;
        if is_valid(mid) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    println!("{}", lo - d - 1);
}
