use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    println!("{}", (0..s.len()).map(|_| 'x').collect::<String>());
}
