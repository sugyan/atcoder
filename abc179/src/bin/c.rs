use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut answer = 0;
    for i in 1..n {
        answer += (1..i).take_while(|j| i * j < n).count() * 2;
        if i * i < n {
            answer += 1;
        }
    }
    println!("{}", answer);
}
