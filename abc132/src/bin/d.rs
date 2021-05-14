use proconio::{fastout, input};

const DIV: u64 = 1_000_000_007;

struct Combinations {
    n: u64,
    fac: Vec<u64>,
    inv: Vec<u64>,
}

impl Combinations {
    fn new(n: u64) -> Self {
        let mut fac = vec![1; n as usize + 1];
        let mut inv = vec![1; n as usize + 1];
        for i in 1..=n {
            fac[i as usize] = (fac[i as usize - 1] * i) % DIV;
        }
        inv[n as usize] = Self::inv(fac[n as usize]);
        for i in (0..n).rev() {
            inv[i as usize] = (inv[i as usize + 1] * (i as u64 + 1)) % DIV;
        }
        Self { n, fac, inv }
    }
    fn nck_mod(&self, k: u64) -> u64 {
        if k > self.n {
            0
        } else {
            (self.fac[self.n as usize]
                * ((self.inv[(self.n - k) as usize] * self.inv[k as usize]) % DIV))
                % DIV
        }
    }
    fn inv(x: u64) -> u64 {
        let mut ret = 1;
        let mut k = DIV - 2;
        let mut y = x;
        while k > 0 {
            if k & 1 > 0 {
                ret = (ret * y) % DIV;
            }
            y = (y * y) % DIV;
            k /= 2;
        }
        ret
    }
}

#[fastout]
fn main() {
    input! {
        n: u64, k: u64,
    }
    let c1 = Combinations::new(n - k + 1);
    let c2 = Combinations::new(k - 1);
    for i in 0..k {
        println!("{}", (c1.nck_mod(i + 1) * c2.nck_mod(i)) % DIV);
    }
}
