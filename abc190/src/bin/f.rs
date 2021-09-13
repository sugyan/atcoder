use proconio::{fastout, input};
use std::ops::AddAssign;

struct BIT<T> {
    v: Vec<T>,
}

impl<T: AddAssign + Copy + Default> BIT<T> {
    fn new(n: usize) -> Self {
        Self {
            v: vec![Default::default(); n + 1],
        }
    }
    fn add(&mut self, i: usize, val: T) {
        let mut x = i;
        while x < self.v.len() {
            self.v[x] += val;
            x += 1 << x.trailing_zeros();
        }
    }
    fn sum(&self, i: usize) -> T {
        let mut ret = Default::default();
        let mut x = i;
        while x > 0 {
            ret += self.v[x];
            x -= 1 << x.trailing_zeros();
        }
        ret
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut bit = BIT::new(n);
    let mut answer = 0;
    for (i, &a) in a.iter().enumerate() {
        answer += i as u64 - bit.sum(a as usize + 1);
        bit.add(a as usize + 1, 1);
    }
    for &a in &a {
        println!("{}", answer);
        answer += n as u64 - 1;
        answer -= a * 2;
    }
}
