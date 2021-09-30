use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u8,
    }
    let answer = match x {
        0..=39 => 40 - x,
        40..=69 => 70 - x,
        70..=89 => 90 - x,
        _ => 0,
    };
    if answer == 0 {
        println!("expert");
    } else {
        println!("{}", answer);
    }
}
