use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        abc: [u32; 3],
    }
    let mut answer = 10_000;
    for i in 0..=(n / abc[0]).min(9999) {
        let m = n - i * abc[0];
        for j in 0..=(m / abc[1]).min(9999 - i) {
            let k = (m - j * abc[1]) / abc[2];
            if j * abc[1] + k * abc[2] == m {
                answer = answer.min(i + j + k);
            }
        }
    }
    println!("{}", answer);
}
