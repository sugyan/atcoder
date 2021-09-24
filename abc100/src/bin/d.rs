use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        xyz: [[i64; 3]; n],
    }
    let mut answer = 0;
    for i in 0..8 {
        let mut v = xyz
            .iter()
            .enumerate()
            .map(|(j, w)| {
                let sum = (0..3)
                    .map(|k| w[k] * if i & 1 << k == 0 { 1 } else { -1 })
                    .sum::<i64>();
                (sum, j)
            })
            .collect::<Vec<_>>();
        v.sort();
        let mut sums = vec![0, 0, 0];
        for j in 0..m {
            for (k, &val) in xyz[v[j].1].iter().enumerate() {
                sums[k] += val;
            }
        }
        answer = answer.max(sums[0].abs() + sums[1].abs() + sums[2].abs());
    }
    println!("{}", answer);
}
