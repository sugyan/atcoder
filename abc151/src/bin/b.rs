use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: i32, m: i32,
        a: [i32; n - 1],
    }
    let answer = n as i32 * m - a.iter().sum::<i32>();
    println!("{}", if answer > k { -1 } else { answer.max(0) });
}
