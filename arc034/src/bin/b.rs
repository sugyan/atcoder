use proconio::{fastout, input};

fn f(x: u64) -> u64 {
    let mut sum = 0;
    let mut x = x;
    while x > 0 {
        sum += x % 10;
        x /= 10;
    }
    sum
}

#[fastout]
fn main() {
    input! {
        n: u64
    }
    let mut answers = Vec::new();
    for i in (1..n).rev() {
        let j = n - i;
        if i + f(i) == n {
            answers.push(i);
        }
        if j as f64 > (i as f64).log10() * 9.0 {
            break;
        }
    }
    println!("{}", answers.len());
    for a in answers.iter().rev() {
        println!("{}", a);
    }
}
