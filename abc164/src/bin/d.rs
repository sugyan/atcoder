use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut v = vec![0; 2019];
    let mut answer = 0;
    for &b in s.as_bytes() {
        let mut tmp = vec![0; 2019];
        tmp[(b - b'0') as usize] += 1;
        for i in 0..2019 {
            tmp[(i * 10 + (b - b'0') as usize) % 2019] += v[i];
        }
        v = tmp;
        answer += v[0];
    }
    println!("{}", answer);
}
