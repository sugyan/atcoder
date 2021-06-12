use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, m: usize,
        hw: [(usize, usize); m],
    }
    let (mut hc, mut wc) = (vec![0; h], vec![0; w]);
    for &(h, w) in &hw {
        hc[h - 1] += 1;
        wc[w - 1] += 1;
    }
    let hmax = *hc.iter().max().unwrap();
    let wmax = *wc.iter().max().unwrap();
    let dup = hw
        .iter()
        .filter(|&(h, w)| hc[h - 1] == hmax && wc[w - 1] == wmax)
        .count()
        >= hc.iter().filter(|&c| *c == hmax).count() * wc.iter().filter(|&c| *c == wmax).count();
    println!("{}", hmax + wmax - if dup { 1 } else { 0 });
}
