use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: u8,
    }
    let mut v = vec!["Christmas"];
    (0..25 - d).for_each(|_| v.push("Eve"));
    println!("{}", v.join(" "));
}
