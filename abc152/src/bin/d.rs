use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut c = [[0; 10]; 10];
    for i in 1..=n {
        let j = (i.to_string().bytes().next().unwrap() - b'0') as usize;
        c[j][i % 10] += 1;
    }
    let mut answer = 0;
    for i in 1..10 {
        for j in 1..10 {
            answer += c[i][j] * c[j][i];
        }
    }
    println!("{}", answer);
}
