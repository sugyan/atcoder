use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
    }
    for i in 1..=3500 {
        for j in 1..=3500 {
            let num = n * i * j;
            let den = 4 * i * j - n * (i + j);
            if den > 0 && num % den == 0 {
                println!("{} {} {}", i, j, num / den);
                return;
            }
        }
    }
}
