use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }
    let yes = (1..n).any(|i| (0..i).any(|j| st[i].0 == st[j].0 && st[i].1 == st[j].1));
    println!("{}", if yes { "Yes" } else { "No" });
}
