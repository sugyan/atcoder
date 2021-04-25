use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; m],
    }
    let mut seen = vec![false; n];
    for a in a.iter().rev() {
        if !seen[a - 1] {
            seen[a - 1] = true;
            println!("{}", a);
        }
    }
    for (i, &b) in seen.iter().enumerate() {
        if !b {
            println!("{}", i + 1);
        }
    }
}
