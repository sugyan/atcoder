use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32, b: u32,
    }
    let yes = (a..=a * 6).contains(&b);
    println!("{}", if yes { "Yes" } else { "No" });
}
