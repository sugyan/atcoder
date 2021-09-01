use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize, m: usize,
        mut ab: [(usize, usize); m],
    }
    ab.sort();
    let mut answer = 0;
    let mut min = 0;
    for &(a, b) in &ab {
        if a < min {
            min = b.min(min);
        } else {
            answer += 1;
            min = b;
        }
    }
    println!("{}", answer);
}
