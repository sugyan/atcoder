use proconio::{fastout, input};

struct BIT {
    v: Vec<u64>,
}

impl BIT {
    fn new(n: usize) -> Self {
        Self { v: vec![0; n + 1] }
    }
    fn add(&mut self, i: usize) {
        let mut x = i;
        while x < self.v.len() {
            self.v[x] += 1;
            x += 1 << x.trailing_zeros();
        }
    }
    fn sum(&self, i: usize) -> u64 {
        let mut ret = 0;
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
        bit.add(a as usize + 1);
    }
    for &a in &a {
        println!("{}", answer);
        answer += n as u64 - 1;
        answer -= a * 2;
    }
}
