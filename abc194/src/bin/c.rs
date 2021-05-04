use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut counts = vec![0; 401];
    a.iter().for_each(|&a| counts[(a + 200) as usize] += 1);
    let mut answer = 0;
    for i in 1..=400 {
        for j in 0..i {
            answer += ((i - j) as u64).pow(2) * counts[i] * counts[j];
        }
    }
    println!("{}", answer);
}
