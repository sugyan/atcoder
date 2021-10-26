use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![0; n + 1];
    for mut i in 1..=n {
        let mut j = 2;
        while j < i {
            while i % j == 0 {
                v[j] += 1;
                i /= j;
            }
            j += 1;
        }
        v[i] += 1;
    }
    let answer = v.iter().skip(2).fold(1, |acc, &x| acc * (x + 1) % MOD);
    println!("{:?}", answer);
}
