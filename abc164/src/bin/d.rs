use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut c = vec![0; 2019];
    c[0] = 1;
    let (mut n, mut p) = (0, 1);
    for &b in s.as_bytes().iter().rev() {
        n = (p * (b - b'0') as i32 + n) % 2019;
        c[n as usize] += 1;
        p = p * 10 % 2019;
    }
    let answer = c.iter().map(|m| m * (m - 1) / 2).sum::<i32>();
    println!("{}", answer);
}
