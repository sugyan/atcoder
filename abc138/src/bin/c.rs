use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut v: [u32; n],
    }
    v.sort_unstable();
    let answer = v.iter().fold(v[0] as f64, |acc, &x| (acc + x as f64) / 2.0);
    println!("{}", answer);
}
