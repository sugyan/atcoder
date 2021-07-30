use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let x = s
        .chars()
        .fold(0, |acc, c| if c == 'g' { acc + 1 } else { acc - 1 });
    println!("{}", x / 2);
}
