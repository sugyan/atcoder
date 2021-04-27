use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
    }
    let answer = if n == 2 || m == 2 {
        0
    } else {
        (n.max(3) - 2) * (m.max(3) - 2)
    };
    println!("{}", answer);
}
