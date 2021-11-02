use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: String,
    }
    let mut stack = Vec::with_capacity(x.len());
    for c in x.chars() {
        if stack.last() == Some(&'S') && c == 'T' {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    println!("{}", stack.len());
}
