use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        mut abc: [u32; 3],
    }
    abc.sort_unstable();
    let mut answer = 10_000;
    for i in 0..=(n / abc[2]).min(9999) {
        let m = n - i * abc[2];
        for j in 0..=(m / abc[1]).min(9999 - i) {
            if (m - j * abc[1]) % abc[0] == 0 {
                answer = answer.min(i + j + (m - j * abc[1]) / abc[0]);
            }
        }
    }
    println!("{}", answer);
}
