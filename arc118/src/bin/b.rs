use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize, n: i64, m: i64,
        a: [i64; k],
    }
    let mut b = a.iter().map(|a| a * m / n).collect::<Vec<_>>();
    let mut v = (0..k).map(|i| (b[i] * n - a[i] * m, i)).collect::<Vec<_>>();
    v.sort();
    for i in 0..(m - b.iter().sum::<i64>()) as usize {
        b[v[i].1] += 1;
    }
    let answers = b.iter().map(|b| b.to_string()).collect::<Vec<_>>();
    println!("{}", answers.join(" "));
}
