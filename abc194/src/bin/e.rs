use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
    }
    let mut counts = vec![0; n];
    (0..m).for_each(|i| counts[a[i]] += 1);
    let mut answer = counts.iter().position(|&c| c == 0).unwrap_or(m);
    for i in 0..n - m {
        counts[a[i + m]] += 1;
        counts[a[i]] -= 1;
        if counts[a[i]] == 0 {
            answer = answer.min(a[i]);
        }
    }
    println!("{}", answer);
}
