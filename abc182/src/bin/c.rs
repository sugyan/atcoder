use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: String,
    }
    let counts = n.as_bytes().iter().fold([0; 3], |mut acc, &b| {
        acc[(b % 3) as usize] += 1;
        acc
    });
    let summod = (counts[1] + counts[2] * 2) % 3;
    let answer = if summod == 0 {
        0
    } else if counts[summod] > 0 && n.len() > 1 {
        1
    } else if n.len() > 2 {
        2
    } else {
        -1
    };
    println!("{}", answer);
}
