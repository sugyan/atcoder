use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    let v0 = xy.iter().map(|&(x, y)| x + y).collect::<Vec<_>>();
    let v1 = xy.iter().map(|&(x, y)| x - y).collect::<Vec<_>>();
    let answer = (v0.iter().max().unwrap() - v0.iter().min().unwrap())
        .max(v1.iter().max().unwrap() - v1.iter().min().unwrap());
    println!("{}", answer);
}
