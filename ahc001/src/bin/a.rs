use proconio::input;
use rand::SeedableRng;
use rand::{rngs::SmallRng, Rng};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(PartialEq, PartialOrd)]
struct NonNan(f64);

impl Eq for NonNan {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }
    fn size(&self) -> i32 {
        (self.x2 - self.x1) * (self.y2 - self.y1)
    }
    fn score(&self, r: i32) -> f64 {
        let s = self.size().min(r) as f64 / self.size().max(r) as f64;
        1.0 - (1.0 - s) * (1.0 - s)
    }
}

fn intersect(r1: &Rect, r2: &Rect) -> bool {
    r1.x2.min(r2.x2) > r1.x1.max(r2.x1) && r1.y2.min(r2.y2) > r1.y1.max(r2.y1)
}

struct Solution {
    n: usize,
    xyr: Vec<(i32, i32, i32)>,
    rng: SmallRng,
}

impl Solution {
    fn new(n: usize, xyr: Vec<(i32, i32, i32)>) -> Self {
        Self {
            n,
            xyr,
            rng: SeedableRng::from_entropy(),
        }
    }
    fn solve(&mut self) -> Vec<Rect> {
        let mut v = Vec::new();
        for &(x, y, _) in self.xyr.iter() {
            v.push(Rect::new(x, y, x + 1, y + 1))
        }
        for &step in [125, 25, 5, 1].iter() {
            let mut bh = BinaryHeap::new();
            for (i, &rect) in v.iter().enumerate() {
                bh.push((Reverse(NonNan(rect.score(self.xyr[i].2))), i));
            }
            while let Some((Reverse(NonNan(score)), i)) = bh.pop() {
                if (1.0 - score).abs() < std::f64::EPSILON {
                    break;
                }
                let rect = v[i];
                let candidates = [
                    Rect::new((rect.x1 - step).max(0), rect.y1, rect.x2, rect.y2),
                    Rect::new(rect.x1, (rect.y1 - step).max(0), rect.x2, rect.y2),
                    Rect::new(rect.x1, rect.y1, (rect.x2 + step).min(10000), rect.y2),
                    Rect::new(rect.x1, rect.y1, rect.x2, (rect.y2 + step).min(10000)),
                ]
                .iter()
                .filter_map(|&next| {
                    if next != rect
                        && next.score(self.xyr[i].2) > score
                        && (0..self.n)
                            .filter(|&j| j != i)
                            .all(|j| !intersect(&v[j], &next))
                    {
                        Some(next)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
                if candidates.is_empty() {
                    bh.push((Reverse(NonNan(1.0)), i));
                    continue;
                }
                let choice = candidates[self.rng.gen_range(0, candidates.len())];
                v[i] = choice;
                bh.push((Reverse(NonNan(choice.score(self.xyr[i].2))), i));
            }
        }
        v
    }
}

fn main() {
    input! {
        n: usize,
        xyr: [(i32, i32, i32); n],
    };
    let mut solution = Solution::new(n, xyr);
    let mut answer = Vec::new();
    let mut max = 0.0;
    for _ in 0..100 {
        let v = solution.solve();
        let total_score = v
            .iter()
            .enumerate()
            .map(|(i, rect)| rect.score(solution.xyr[i].2))
            .sum::<f64>();
        if total_score > max {
            max = total_score;
            answer = v;
        }
    }
    for &rect in &answer {
        println!("{} {} {} {}", rect.x1, rect.y1, rect.x2, rect.y2);
    }
}
