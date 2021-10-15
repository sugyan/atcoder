use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut answers = vec![0; n];
    for x in 1..100 {
        for y in 1..100 {
            for z in 1..100 {
                let i = x * x + y * y + z * z + x * y + y * z + z * x;
                if i <= n {
                    answers[i - 1] += 1;
                }
            }
        }
    }
    for a in answers {
        println!("{}", a);
    }
}
