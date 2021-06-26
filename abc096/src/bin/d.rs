use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = Vec::with_capacity(n);
    let mut i = 1;
    while v.len() < n {
        i += 10;
        if (2..i >> 1).all(|j| i % j != 0) {
            v.push(i.to_string());
        }
    }
    println!("{}", v.join(" "));
}
