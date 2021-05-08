use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }
    let mut v = vec![0; 200];
    a.iter().for_each(|&a| v[(a % 200) as usize] += 1_i64);
    let answer = v.iter().map(|&e| e * (e - 1) / 2).sum::<i64>();
    println!("{}", answer);
}
