use proconio::{fastout, input};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        _h: usize, _w: usize, n: usize,
        s: (usize, usize),
        g: (usize, usize),
        xy: [(usize, usize); n],
    }
    let (mut xhm, mut yhm) = (HashMap::new(), HashMap::new());
    for &(x, y) in &xy {
        xhm.entry(x).or_insert_with(BTreeSet::new).insert(y);
        yhm.entry(y).or_insert_with(BTreeSet::new).insert(x);
    }
    let mut vd = VecDeque::new();
    vd.push_back((s, 0));
    let mut visited = HashSet::new();
    while let Some(((x, y), step)) = vd.pop_front() {
        if (x, y) == g {
            println!("{}", step);
            return;
        }
        if let Some(ys) = xhm.get(&x) {
            if let Some(oy) = ys.range(..y).rev().next() {
                if oy + 1 < y && !visited.contains(&(x, oy + 1)) {
                    visited.insert((x, oy + 1));
                    vd.push_back(((x, oy + 1), step + 1));
                }
            }
            if let Some(oy) = ys.range(y + 1..).next() {
                if oy - 1 > y && !visited.contains(&(x, oy - 1)) {
                    visited.insert((x, oy - 1));
                    vd.push_back(((x, oy - 1), step + 1));
                }
            }
        }
        if let Some(xs) = yhm.get(&y) {
            if let Some(ox) = xs.range(..x).rev().next() {
                if ox + 1 < x && !visited.contains(&(ox + 1, y)) {
                    visited.insert((ox + 1, y));
                    vd.push_back(((ox + 1, y), step + 1));
                }
            }
            if let Some(ox) = xs.range(x + 1..).next() {
                if ox - 1 > x && !visited.contains(&(ox - 1, y)) {
                    visited.insert((ox - 1, y));
                    vd.push_back(((ox - 1, y), step + 1));
                }
            }
        }
    }
    println!("-1");
}
