use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let r = s.chars().collect::<Vec<_>>();
    let mut answer = 0;
    for i in 0..10_000 {
        let mut c = [0; 10];
        for &u in format!("{:04}", i).as_bytes() {
            c[(u - b'0') as usize] += 1;
        }
        if !(0..10).any(|j| (r[j] == 'o' && c[j] == 0) || (r[j] == 'x' && c[j] > 0)) {
            answer += 1;
        }
    }
    println!("{}", answer);
}
