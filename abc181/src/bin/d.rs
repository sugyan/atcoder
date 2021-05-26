use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let counts = |s: &str| {
        s.as_bytes().iter().fold([0; 10], |mut acc, u| {
            acc[(u - b'0') as usize] += 1;
            acc
        })
    };
    let s = counts(&s);
    let answer = (0..1000).step_by(8).any(|n| {
        let c = counts(&n.to_string());
        if n < 100 {
            s == c
        } else {
            (0..10).all(|i| s[i] >= c[i])
        }
    });
    println!("{}", if answer { "Yes" } else { "No" });
}
