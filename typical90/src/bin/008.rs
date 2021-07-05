use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }
    let mut v = [0; 7];
    for c in s.chars() {
        match c {
            'a' => v[0] += 1,
            't' => v[1] = (v[0] + v[1]) % MOD,
            'c' => v[2] = (v[1] + v[2]) % MOD,
            'o' => v[3] = (v[2] + v[3]) % MOD,
            'd' => v[4] = (v[3] + v[4]) % MOD,
            'e' => v[5] = (v[4] + v[5]) % MOD,
            'r' => v[6] = (v[5] + v[6]) % MOD,
            _ => {}
        }
    }
    println!("{:?}", v[6] % MOD);
}
