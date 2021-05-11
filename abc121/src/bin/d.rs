use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: u64, b: u64,
    }
    if a > 0 {
        a -= 1;
    }
    let mut answer = 0;
    for i in 0..64 {
        let j = 1 << i;
        let na = a / j / 2 * j + if (a / j) % 2 == 1 { a % j + 1 } else { 0 };
        let nb = b / j / 2 * j + if (b / j) % 2 == 1 { b % j + 1 } else { 0 };
        if (nb - na) % 2 == 1 {
            answer += j;
        }
    }
    println!("{}", answer);
}
