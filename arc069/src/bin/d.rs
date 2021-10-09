use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut candidates = [
        vec!['S', 'S'],
        vec!['S', 'W'],
        vec!['W', 'S'],
        vec!['W', 'W'],
    ];
    for v in candidates.iter_mut() {
        for i in 1..=n {
            let same = matches!((s[i % n], v[i]), ('o', 'S') | ('x', 'W'));
            v.push(if same {
                v[i - 1]
            } else if v[i - 1] == 'S' {
                'W'
            } else {
                'S'
            });
        }
        if v[n] == v[0] && v[n + 1] == v[1] {
            v.pop();
            v.pop();
            return println!("{}", v.iter().collect::<String>());
        }
    }
    println!("-1");
}
