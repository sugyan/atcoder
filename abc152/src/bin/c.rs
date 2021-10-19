use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [u32; n],
    }
    let mut min = std::u32::MAX;
    let mut answer = 0;
    for &p in &p {
        min = min.min(p);
        if p == min {
            answer += 1;
        }
    }
    println!("{}", answer)
}
