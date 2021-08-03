use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut b: [usize; n],
    }
    let mut answers = Vec::with_capacity(n);
    while let Some(i) = b.iter().enumerate().rposition(|(i, b)| *b == i + 1) {
        answers.push(i + 1);
        b.remove(i);
    }
    if !b.is_empty() {
        println!("-1");
        return;
    }
    for a in answers.iter().rev() {
        println!("{}", a);
    }
}
