use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        ab: (i32, i32),
        cd: (i32, i32),
    }
    println!("{}", ab.1 - cd.0);
}
