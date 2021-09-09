use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, q: usize,
        mut lr: [(usize, usize); m],
        pq: [(usize, usize); q],
    }
    let mut v = vec![Vec::new(); n];
    lr.sort();
    for &(l, r) in &lr {
        v[l - 1].push(r - 1);
    }
    for &(p, q) in &pq {
        let mut answer = 0;
        for v in &v[p - 1..q] {
            answer += match v.binary_search(&(q - 1)) {
                Ok(j) => j + 1,
                Err(j) => j,
            }
        }
        println!("{}", answer);
    }
}
