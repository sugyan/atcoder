use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64
    }
    let mut answers = Vec::new();
    for x in n.max(200) - 200..n {
        let mut sum = x;
        let mut y = x;
        while y > 0 {
            sum += y % 10;
            y /= 10;
        }
        if sum == n {
            answers.push(x);
        }
    }
    println!("{}", answers.len());
    for a in &answers {
        println!("{}", a);
    }
}
