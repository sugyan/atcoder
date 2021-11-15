use proconio::{fastout, input};

struct XorSegTree {
    offset: usize,
    v: Vec<u32>,
}

impl XorSegTree {
    fn new(size: usize) -> Self {
        let n = size.next_power_of_two();
        Self {
            offset: n,
            v: vec![0; n << 1],
        }
    }
    fn query(&self, lo: usize, hi: usize) -> u32 {
        self.sub_query(lo, hi + 1, 0, 0, self.offset)
    }
    fn sub_query(&self, lo: usize, hi: usize, pos: usize, min: usize, max: usize) -> u32 {
        if max <= lo || hi <= min {
            0
        } else if lo <= min && max <= hi {
            self.v[pos]
        } else {
            let vl = self.sub_query(lo, hi, pos * 2 + 1, min, (min + max) / 2);
            let vh = self.sub_query(lo, hi, pos * 2 + 2, (min + max) / 2, max);
            vl ^ vh
        }
    }
    fn update(&mut self, pos: usize, val: u32) {
        let mut i = pos + self.offset - 1;
        self.v[i] = val;
        while i > 0 {
            i = (i - 1) >> 1;
            self.v[i] = self.v[(i << 1) + 1] ^ self.v[(i << 1) + 2]
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        mut a: [u32; n],
        txy: [(u8, usize, usize); q],
    }
    let mut xst = XorSegTree::new(n);
    for (i, &a) in a.iter().enumerate() {
        xst.update(i, a);
    }
    for &(t, x, y) in &txy {
        if t == 1 {
            a[x - 1] ^= y as u32;
            xst.update(x - 1, a[x - 1]);
        } else {
            println!("{}", xst.query(x - 1, y - 1));
        }
    }
}
