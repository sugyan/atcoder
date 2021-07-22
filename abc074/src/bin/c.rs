use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32, b: u32, c: u32, d: u32, e: u32, f: u32,
    }
    let (mut maxval, mut answer) = (0.0, (a * 100, 0));
    for ia in 0..=f / a / 100 {
        for ib in 0..=(f - a * ia * 100) / b / 100 {
            let water = a * ia * 100 + b * ib * 100;
            let limit = ((ia * a + ib * b) * e).min(f - water);
            let mut max = 0;
            for ic in 0..=limit / c {
                let id = (limit - c * ic) / d;
                max = max.max(ic * c + id * d);
            }
            if max + water > 0 {
                let concentration = max as f64 / (max + water) as f64;
                if concentration > maxval {
                    maxval = concentration;
                    answer = (water + max, max);
                }
            }
        }
    }
    println!("{} {}", answer.0, answer.1);
}
