use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, mut k: u64,
        a: [u32; n],
    }
    let mut answers = vec![k / n as u64; n];
    let mut v = a.iter().enumerate().collect::<Vec<_>>();
    v.sort_by_key(|(_, &a)| a);
    for &(i, _) in v.iter().take(k as usize % n) {
        answers[i] += 1;
    }
    for a in &answers {
        println!("{}", a);
    }
}
