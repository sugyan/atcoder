use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [u32; n],
        q: usize,
        k: [usize; q],
    }
    let mut s = s.iter().filter(|&s| *s > 0).collect::<Vec<_>>();
    s.sort();
    s.reverse();
    for &k in &k {
        println!("{}", if k < s.len() { s[k] + 1 } else { 0 });
    }
}
