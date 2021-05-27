use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        mut h: [u32; n],
        mut w: [u32; m],
    }
    h.sort_unstable();
    w.sort_unstable();
    let d = |x: u32| match w.binary_search(&x) {
        Ok(_) => 0,
        Err(i) => {
            let mut ret = if i > 0 { x - w[i - 1] } else { w[0] - x };
            if i < m {
                ret = ret.min(w[i] - x);
            }
            ret
        }
    };
    let mut sum = (0..n / 2).map(|i| h[i * 2 + 2] - h[i * 2 + 1]).sum::<u32>();
    let mut answer = sum + d(h[0]);
    for i in 0..n / 2 {
        sum -= h[i * 2 + 2] - h[i * 2 + 1];
        sum += h[i * 2 + 1] - h[i * 2];
        answer = answer.min(sum + d(h[i * 2 + 2]));
    }
    println!("{}", answer);
}
