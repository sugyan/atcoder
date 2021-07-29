use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ta: [(u64, u64); n],
    }
    let mut m = (1, 1);
    for &(t, a) in &ta {
        let f = ((m.0 + t - 1) / t).max((m.1 + a - 1) / a);
        m = (t * f, a * f);
    }
    println!("{}", m.0 + m.1);
}
