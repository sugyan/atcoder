use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        mut ab: [(i32, usize); n],
    }
    ab.sort_unstable();
    let mut i = 0;
    for &(a, b) in &ab {
        i += b;
        if i >= k {
            println!("{}", a);
            break;
        }
    }
}
