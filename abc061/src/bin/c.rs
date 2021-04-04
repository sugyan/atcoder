use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        mut ab: [(i32, usize); n],
    }
    ab.sort_unstable();
    let mut answer = ab[0].0;
    let mut i = 0;
    for &(a, b) in &ab {
        answer = a;
        i += b;
        if i >= k {
            break;
        }
    }
    println!("{}", answer);
}
