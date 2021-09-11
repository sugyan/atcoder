use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, s: u32, d: u32,
        xy: [(u32, u32); n],
    }
    let yes = xy.iter().any(|&(x, y)| x < s && y > d);
    println!("{}", if yes { "Yes" } else { "No" });
}
