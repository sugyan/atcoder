use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (r1, c1): (i64, i64),
        (r2, c2): (i64, i64),
    }
    let answer = if r1 == r2 && c1 == c2 {
        0
    } else if r1 + c1 == r2 + c2 || r1 - c1 == r2 - c2 || (r1 - r2).abs() + (c1 - c2).abs() <= 3 {
        1
    } else if (r1 + c1) % 2 == (r2 + c2) % 2
        || ((r1 + c1) - (r2 + c2)).abs() <= 3
        || ((r1 - c1) - (r2 - c2)).abs() <= 3
        || (r1 - r2).abs() + (c1 - c2).abs() <= 6
    {
        2
    } else {
        3
    };
    println!("{}", answer);
}
