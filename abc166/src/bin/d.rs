use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
    }
    for a in 0_i64..=119 {
        for b in -119_i64..=119 {
            if a.pow(5) - b.pow(5) == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}
