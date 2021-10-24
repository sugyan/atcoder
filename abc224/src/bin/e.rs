use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, n: usize,
        rca: [(usize, usize, u32); n],
    }
    let mut btm = BTreeMap::new();
    for (i, &(_, _, a)) in rca.iter().enumerate() {
        btm.entry(a).or_insert_with(Vec::new).push(i);
    }
    let mut answers = vec![0; n];
    let mut rmax = vec![0; h];
    let mut cmax = vec![0; w];
    for v in btm.values().rev() {
        for &i in v {
            answers[i] = rmax[rca[i].0 - 1].max(cmax[rca[i].1 - 1]);
        }
        for &i in v {
            rmax[rca[i].0 - 1] = rmax[rca[i].0 - 1].max(answers[i] + 1);
            cmax[rca[i].1 - 1] = cmax[rca[i].1 - 1].max(answers[i] + 1);
        }
    }
    for a in &answers {
        println!("{}", a);
    }
}
