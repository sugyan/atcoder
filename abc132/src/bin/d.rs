use proconio::{fastout, input};

const DIV: u64 = 1_000_000_007;

fn f(n: u64) -> Vec<u64> {
    let mut v = vec![0; n as usize + 1];
    v[0] = 1;
    for i in 0..n as usize {
        for j in (0..=i).rev() {
            v[j + 1] = (v[j + 1] + v[j]) % DIV;
        }
    }
    v
}

#[fastout]
fn main() {
    input! {
        n: u64, k: u64,
    }
    let vs = (f(n - k + 1), f(k - 1));
    for i in 0..k as usize {
        let answer = if i + 1 < vs.0.len() {
            (vs.0[i + 1] * vs.1[i]) % DIV
        } else {
            0
        };
        println!("{}", answer);
    }
}
