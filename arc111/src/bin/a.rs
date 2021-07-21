use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64, m: u64,
    }
    let mut x = 10;
    let mut y = n;
    let mut z = 1;
    while y > 0 {
        if y & 1 != 0 {
            z = (z * x) % (m * m);
        }
        x = (x * x) % (m * m);
        y >>= 1;
    }
    println!("{}", z / m % m);
}
