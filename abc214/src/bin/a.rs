use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8,
    }
    let answer = match n {
        1..=125 => 4,
        126..=211 => 6,
        212..=214 => 8,
        _ => unreachable!(),
    };
    println!("{}", answer)
}
