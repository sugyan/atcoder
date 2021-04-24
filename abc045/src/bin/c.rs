use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let s = s
        .as_bytes()
        .iter()
        .map(|&b| (b - b'0') as u64)
        .collect::<Vec<_>>();
    let mut answer = 0;
    for i in 0..1 << (s.len() - 1) {
        let mut n = s[0];
        for j in 0..s.len() - 1 {
            if (i >> j) & 1 == 0 {
                answer += n;
                n = 0;
            } else {
                n *= 10;
            }
            n += s[j + 1];
        }
        answer += n;
    }
    println!("{}", answer);
}
