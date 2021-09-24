use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        xyz: [[i64; 3]; n],
    }
    let mut answer = 0;
    for i in 0..8 {
        let s = (0..3)
            .map(|j| if i & 1 << j == 0 { 1 } else { -1 })
            .collect::<Vec<_>>();
        let mut v = xyz
            .iter()
            .map(|w| w.iter().zip(&s).map(|(val, sign)| val * sign).sum::<i64>())
            .collect::<Vec<_>>();
        v.sort();
        answer = answer.max(v[n - m..].iter().sum::<i64>());
    }
    println!("{}", answer);
}
