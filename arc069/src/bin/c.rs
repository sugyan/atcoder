use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64, m: u64
    }
    let mut answer = n.min(m / 2);
    answer += (m - answer * 2) / 4;
    println!("{}", answer)
}
