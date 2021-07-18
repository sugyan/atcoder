use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: String,
    }
    println!("{}", t.replace('?', "D"));
}
