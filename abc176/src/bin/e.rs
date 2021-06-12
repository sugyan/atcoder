use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, m: usize,
        hw: [(usize, usize); m],
    }
    let (mut hc, mut wc) = (vec![0; h], vec![0; w]);
    let (mut hmax, mut wmax) = (0, 0);
    let (mut hv, mut wv) = (Vec::new(), Vec::new());
    for &(h, w) in &hw {
        hc[h - 1] += 1;
        wc[w - 1] += 1;
        if hc[h - 1] >= hmax {
            if hc[h - 1] > hmax {
                hmax = hc[h - 1];
                hv.clear();
            }
            hv.push(h - 1);
        }
        if wc[w - 1] >= wmax {
            if wc[w - 1] > wmax {
                wmax = wc[w - 1];
                wv.clear();
            }
            wv.push(w - 1);
        }
    }
    let hw = hw.iter().collect::<HashSet<_>>();
    let mut answer = hc[hv[0]] + wc[wv[0]];
    if hv
        .iter()
        .flat_map(|i| std::iter::repeat(i).zip(&wv))
        .all(|(&i, &j)| hw.contains(&(i + 1, j + 1)))
    {
        answer -= 1;
    }
    println!("{}", answer);
}
