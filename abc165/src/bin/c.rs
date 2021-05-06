use proconio::{fastout, input};

fn gen(a: &mut Vec<Vec<u8>>, v: &mut Vec<u8>, n: u8, m: u8) {
    if v.len() == n as usize {
        a.push(v.clone());
        return;
    }
    for i in *v.last().unwrap_or(&1)..=m {
        v.push(i);
        gen(a, v, n, m);
        v.pop();
    }
}

#[fastout]
fn main() {
    input! {
        n: u8, m: u8, q: usize,
        abcd: [(usize, usize, u8, u32); q],
    }
    let mut a = Vec::new();
    let mut v = Vec::new();
    gen(&mut a, &mut v, n, m);
    let mut answer = 0;
    for v in &a {
        let mut score = 0;
        for &(a, b, c, d) in &abcd {
            if v[b - 1] - v[a - 1] == c {
                score += d;
            }
        }
        answer = answer.max(score);
    }
    println!("{:?}", answer);
}
