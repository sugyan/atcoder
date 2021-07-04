use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut c = [0_u64; 5];
    for s in &s {
        match s.as_bytes()[0] {
            b'M' => c[0] += 1,
            b'A' => c[1] += 1,
            b'R' => c[2] += 1,
            b'C' => c[3] += 1,
            b'H' => c[4] += 1,
            _ => {}
        }
    }
    let mut answer = 0;
    for i in 0..3 {
        for j in i + 1..4 {
            for k in j + 1..5 {
                answer += c[i] * c[j] * c[k];
            }
        }
    }
    println!("{}", answer);
}
