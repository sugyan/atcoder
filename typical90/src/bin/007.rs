use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
        q: usize,
        b: [u32; q],
    }
    a.sort_unstable();
    for b in &b {
        let answer = match a.binary_search(b) {
            Ok(_) => 0,
            Err(i) => match i {
                0 => a[0] - b,
                i if i == a.len() => b - a[i - 1],
                i => (a[i] - b).min(b - a[i - 1]),
            },
        };
        println!("{}", answer);
    }
}
