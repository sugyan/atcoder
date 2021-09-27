use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32, b: u32, c: u32
    }
    if (a - 1) / c + 1 > b / c {
        println!("-1");
    } else {
        println!("{}", b / c * c);
    }
}
