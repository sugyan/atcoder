use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }
    let mut answer = 0;
    for i in 0..1000 {
        let p = format!("{:03}", i).as_bytes().to_owned();
        let mut j = 0;
        for &u in s.as_bytes() {
            if j < 3 && u == p[j] {
                j += 1;
            }
        }
        if j == 3 {
            answer += 1;
        }
    }
    println!("{}", answer);
}
