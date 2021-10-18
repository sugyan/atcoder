use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let max = 1_000_000;
    let mut sieve = (0..=max).collect::<Vec<_>>();
    for i in 2..=1000 {
        if sieve[i] == i {
            (i..=max).step_by(i).for_each(|j| sieve[j] = i);
        }
    }
    let mut counts = vec![0; max + 1];
    for &a in &a {
        let mut a = a as usize;
        while a > 1 {
            let m = sieve[a];
            let mut c = 0;
            while a % m == 0 {
                a /= m;
                c += 1;
            }
            counts[m] = counts[m].max(c);
        }
    }
    let mut lcm = 1;
    for (i, &c) in counts.iter().enumerate() {
        (0..c).for_each(|_| lcm = lcm * i as u64 % MOD);
    }
    let answer = a.iter().map(|&a| lcm * pow(a, MOD - 2) % MOD).sum::<u64>() % MOD;
    println!("{}", answer);
}

fn pow(a: u64, b: u64) -> u64 {
    let mut ret = 1;
    let mut a = a;
    let mut b = b;
    while b > 0 {
        if b & 1 > 0 {
            ret = ret * a % MOD
        }
        a = a * a % MOD;
        b >>= 1;
    }
    ret
}
