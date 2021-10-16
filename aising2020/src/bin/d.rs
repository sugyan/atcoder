use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: Chars,
    }
    let count = x.iter().filter(|&c| *c == '1').count();
    if count == 0 {
        for _ in 0..n {
            println!("1");
        }
        return;
    }
    if count == 1 {
        let answers = vec![if x[n - 1] == '1' { 2 } else { 1 }; n];
        for (i, &a) in answers.iter().enumerate().take(n - 1) {
            println!("{}", if x[i] == '1' { 0 } else { a });
        }
        println!("{}", if x[n - 1] == '1' { 0 } else { 2 });
        return;
    }
    let mut m = vec![vec![1; n]; 2];
    for i in (0..n - 1).rev() {
        m[0][i] = (m[0][i + 1] * 2) % (count + 1);
        m[1][i] = (m[1][i + 1] * 2) % (count - 1);
    }
    let mut sums = vec![0; 2];
    for (i, &c) in x.iter().enumerate() {
        if c == '1' {
            sums[0] = (sums[0] + m[0][i]) % (count + 1);
            sums[1] = (sums[1] + m[1][i]) % (count - 1);
        }
    }
    let mut v = (0..n)
        .map(|i| {
            if x[i] == '0' {
                (sums[0] + m[0][i]) % (count + 1)
            } else {
                (sums[1] + count - 1 - m[1][i]) % (count - 1)
            }
        })
        .collect::<Vec<_>>();
    let mut answers = vec![1; n];
    while v.iter().any(|&e| e > 0) {
        for i in 0..n {
            if v[i] > 0 {
                answers[i] += 1;
                v[i] = v[i] % v[i].count_ones() as usize;
            }
        }
    }
    for a in answers {
        println!("{}", a);
    }
}
