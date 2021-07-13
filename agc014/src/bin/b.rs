use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    }
    let mut c = vec![0; n];
    for &(a, b) in &ab {
        c[a - 1] += 1;
        c[b - 1] += 1;
    }
    let yes = c.iter().all(|m| m & 1 == 0);
    println!("{}", if yes { "YES" } else { "NO" });
}
