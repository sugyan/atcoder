use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: u32, w: u32,
    }
    let answer = if h == 1 || w == 1 {
        h * w
    } else {
        ((h + 1) / 2) * ((w + 1) / 2)
    };
    println!("{}", answer);
}
