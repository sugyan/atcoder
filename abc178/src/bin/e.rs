use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    let mut m = [0, std::i32::MAX, std::i32::MIN, std::i32::MAX];
    for &(x, y) in &xy {
        m[0] = m[0].max(x + y);
        m[1] = m[1].min(x + y);
        m[2] = m[2].max(x - y);
        m[3] = m[3].min(x - y);
    }
    println!("{}", (m[0] - m[1]).max((m[2] - m[3]).abs()));
}
