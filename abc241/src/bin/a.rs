use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [usize; 10],
    }
    let mut answer = 0;
    for _ in 0..3 {
        answer = a[answer];
    }
    println!("{}", answer);
}
