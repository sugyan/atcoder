use proconio::{fastout, input};

fn main() {
    input! {
        n: usize,
    }
    if n % 2 == 0 {
        let mut v = Vec::with_capacity(n);
        backtrack(&mut v, 0, n / 2);
    }
}

#[fastout]
fn backtrack(v: &mut Vec<char>, i: usize, j: usize) {
    if i + j == 0 {
        println!("{}", v.iter().collect::<String>());
        return;
    }
    if j > 0 {
        v.push('(');
        backtrack(v, i + 1, j - 1);
        v.pop();
    }
    if i > 0 {
        v.push(')');
        backtrack(v, i - 1, j);
        v.pop();
    }
}
