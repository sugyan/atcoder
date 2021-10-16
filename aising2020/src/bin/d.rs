use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: Chars,
    }
    let count = x.iter().filter(|&c| *c == '1').count();
    let mut m = vec![vec![1; n]; 2];
    for i in (0..n - 1).rev() {
        m[0][i] = (m[0][i + 1] * 2) % (count + 1);
        m[1][i] = (m[1][i + 1] * 2) % (count.max(2) - 1);
    }
    let mut sums = vec![0; 2];
    for (i, _) in x.iter().enumerate().filter(|(_, &c)| c == '1') {
        sums[0] = (sums[0] + m[0][i]) % (count + 1);
        sums[1] = (sums[1] + m[1][i]) % (count.max(2) - 1);
    }
    for (i, &c) in x.iter().enumerate() {
        if count == 0 || (count == 1 && c == '1') {
            println!("{}", 1 - count);
            continue;
        }
        let mut v = if c == '0' {
            (sums[0] + m[0][i]) % (count + 1)
        } else {
            (sums[1] + count - 1 - m[1][i]) % (count - 1)
        };
        let mut answer = 1;
        while v > 0 {
            v = v % v.count_ones() as usize;
            answer += 1;
        }
        println!("{}", answer);
    }
}
