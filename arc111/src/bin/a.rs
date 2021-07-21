use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64, m: u64,
    }
    println!("{}", pow(10, n, m * m) / m % m);
}

fn pow(x: u64, y: u64, m: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    let mut ret = 1;
    while y > 0 {
        if y & 1 != 0 {
            ret = (ret * x) % m;
        }
        x = (x * x) % m;
        y >>= 1;
    }
    ret
}
