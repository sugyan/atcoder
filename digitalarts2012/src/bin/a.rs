use proconio::marker::Chars;
use proconio::{fastout, input};
use std::io::BufRead;

#[fastout]
fn main() {
    let mut s = String::new();
    std::io::stdin().lock().read_line(&mut s).ok();
    input! {
        n: usize,
        t: [Chars; n],
    }
    let is_ng = |word: &[char]| -> bool {
        t.iter().any(|filter| {
            filter.len() == word.len()
                && (0..filter.len()).all(|i| filter[i] == '*' || filter[i] == word[i])
        })
    };
    let mut answers = Vec::new();
    for word in s.trim().split(' ') {
        let cs = word.chars().collect::<Vec<_>>();
        answers.push(if is_ng(&cs) {
            "*".repeat(word.len())
        } else {
            word.to_string()
        });
    }
    println!("{}", answers.join(" "));
}
