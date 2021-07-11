use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    let mut m = [[0; 46]; 3];
    for (i, v) in [a, b, c].iter().enumerate() {
        for e in v {
            m[i][e % 46] += 1;
        }
    }
    let mut answer = 0_i64;
    for i in 0..46 {
        for j in 0..46 {
            answer += m[0][i] * m[1][j] * m[2][(46 - (i + j) % 46) % 46];
        }
    }
    println!("{}", answer);
}
