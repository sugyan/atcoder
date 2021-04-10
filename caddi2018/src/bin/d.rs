use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    if a.iter().all(|&m| m % 2 == 0) {
        println!("second");
    } else {
        println!("first");
    }
}
