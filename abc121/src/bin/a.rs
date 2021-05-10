use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        hw: [(usize, usize); 2],
    }
    println!("{}", (hw[0].0 - hw[1].0) * (hw[0].1 - hw[1].1));
}
