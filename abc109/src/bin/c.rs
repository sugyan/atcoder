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
    let answer = xs
        .iter()
        .map(|u| (u - x).abs())
        .fold(None, |acc, x| Some(acc.map_or(x, |a| gcd(a, x))));
    println!("{}", answer.unwrap());
}
