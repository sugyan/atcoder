use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32, m: usize, t: i32,
        ab: [(i32, i32); m],
    }
    let (mut val, mut p) = (n, 0);
    let mut yes = true;
    for &(a, b) in &ab {
        val -= a - p;
        yes &= val > 0;
        val += (b - a).min(n - val);
        p = b;
    }
    val -= t - p;
    yes &= val > 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
