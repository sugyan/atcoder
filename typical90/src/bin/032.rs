use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[u32; n]; n],
        m: usize,
        xy: [(usize, usize); m],
    }
    let mut d = vec![Vec::new(); n];
    for &(x, y) in &xy {
        d[x - 1].push(y - 1);
        d[y - 1].push(x - 1);
    }
    let mut v = Vec::with_capacity(n);
    if let Some(answer) = backtrack(&a, &d, &mut v, None, 0) {
        println!("{}", answer);
    } else {
        println!("-1");
    }
}

fn backtrack(
    a: &[Vec<u32>],
    d: &[Vec<usize>],
    v: &mut Vec<usize>,
    prev: Option<usize>,
    t: u32,
) -> Option<u32> {
    if v.len() == a.len() {
        return Some(t);
    }
    let mut ret = None;
    for i in 0..a.len() {
        if v.contains(&i) {
            continue;
        }
        if let Some(p) = prev {
            if p == i || d[i].contains(&p) {
                continue;
            }
        }
        v.push(i);
        if let Some(t) = backtrack(a, d, v, Some(i), t + a[i][v.len() - 1]) {
            ret = Some(ret.map_or(t, |min: u32| min.min(t)));
        }
        v.pop();
    }
    ret
}
