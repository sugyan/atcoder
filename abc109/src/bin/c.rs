use proconio::{fastout, input};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize, x: i32,
        xs: [i32; n],
    }
    println!("{}", xs.iter().map(|u| (u - x).abs()).fold(0, gcd));
}
