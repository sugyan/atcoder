use proconio::{fastout, input};

struct MaxSegTree<T> {
    offset: usize,
    v: Vec<T>,
}

impl<T: Copy + Ord + Default> MaxSegTree<T> {
    fn new(size: usize) -> Self {
        let n = size.next_power_of_two();
        Self {
            offset: n,
            v: vec![Default::default(); n << 1],
        }
    }
    fn query(&self, lo: usize, hi: usize) -> T {
        self.sub_query(lo, hi + 1, 0, 0, self.offset)
    }
    fn sub_query(&self, lo: usize, hi: usize, pos: usize, min: usize, max: usize) -> T {
        if max <= lo || hi <= min {
            Default::default()
        } else if lo <= min && max <= hi {
            self.v[pos]
        } else {
            let vl = self.sub_query(lo, hi, pos * 2 + 1, min, (min + max) / 2);
            let vh = self.sub_query(lo, hi, pos * 2 + 2, (min + max) / 2, max);
            vl.max(vh)
        }
    }
    fn update(&mut self, pos: usize, val: T) {
        let mut i = pos + self.offset - 1;
        self.v[i] = val;
        while i > 0 {
            i = (i - 1) >> 1;
            self.v[i] = self.v[i].max(val);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize, k: i32,
        a: [i32; n],
    }
    let mut mst = MaxSegTree::<i32>::new(300_001);
    for &a in &a {
        let max = mst.query(0.max(a - k) as usize, 300_001.min(a + k) as usize);
        mst.update(a as usize, max + 1);
    }
    println!("{}", mst.query(0, 300_000));
}
