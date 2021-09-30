use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: String,
        n: usize,
        mut s: [String; n],
    }
    let map = x.bytes().enumerate().fold([0; 26], |mut acc, (i, u)| {
        acc[(u - b'a') as usize] = i;
        acc
    });
    s.sort_by_cached_key(|s| {
        s.bytes()
            .map(|u| map[(u - b'a') as usize])
            .collect::<Vec<_>>()
    });
    for s in &s {
        println!("{}", s);
    }
}
