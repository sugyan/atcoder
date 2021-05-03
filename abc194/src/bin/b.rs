use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(u32, u32); n],
    }
    let mut answer = ab.iter().map(|&(a, b)| a + b).min().unwrap();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                answer = answer.min(ab[i].0.max(ab[j].1));
            }
        }
    }
    println!("{}", answer);
}
