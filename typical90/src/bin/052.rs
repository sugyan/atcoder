use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[u64; 6]; n],
    }
    let mut v = a[0].clone();
    for a in a.iter().skip(1) {
        v = a
            .iter()
            .map(|a| v.iter().map(|v| v * a).sum::<u64>() % MOD)
            .collect();
    }
    println!("{}", v.iter().sum::<u64>() % MOD);
}
