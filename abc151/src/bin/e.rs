use proconio::{fastout, input};

const MOD: i64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        mut a: [i64; n],
    }
    a.sort();
    let mut fact = vec![1; n + 1];
    for i in 0..n {
        fact[i + 1] = fact[i] * (i as i64 + 1) % MOD;
    }
    let mut fact_inv = vec![0; n + 1];
    fact_inv[n] = pow(fact[n], MOD - 2);
    for i in (0..n).rev() {
        fact_inv[i] = fact_inv[i + 1] * (i as i64 + 1) % MOD;
    }
    let ncr = |n, r| fact[n] * fact_inv[r] % MOD * fact_inv[n - r] % MOD;
    let mut answer = 0;
    for (i, &val) in a.iter().enumerate().skip(k - 1) {
        answer = (answer + val * ncr(i, k - 1)).rem_euclid(MOD);
    }
    for (i, &val) in a.iter().enumerate().take(n - k + 1) {
        answer = (answer - val * ncr(n - i - 1, k - 1)).rem_euclid(MOD);
    }
    println!("{}", answer);
}

fn pow(mut a: i64, mut b: i64) -> i64 {
    let mut ret = 1;
    while b > 0 {
        if b & 1 > 0 {
            ret = ret * a % MOD
        }
        a = a * a % MOD;
        b >>= 1;
    }
    ret
}
