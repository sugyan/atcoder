use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let result = s
        .as_bytes()
        .iter()
        .fold([0; 26], |mut acc, &u| {
            acc[(u - b'A') as usize] += 1;
            acc
        })
        .iter()
        .all(|&n| n == 0 || n == 2);
    println!("{}", if result { "Yes" } else { "No" });
}
