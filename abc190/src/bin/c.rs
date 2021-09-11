use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
        k: usize,
        cd: [(usize, usize); k],
    }
    let mut answer = 0;
    for i in 0..1 << k {
        let mut v = vec![false; n];
        for (j, (c, d)) in cd.iter().enumerate() {
            v[if (1 << j) & i > 0 { c - 1 } else { d - 1 }] = true;
        }
        answer = answer.max(ab.iter().filter(|&(a, b)| v[a - 1] && v[b - 1]).count());
    }
    println!("{}", answer);
}
