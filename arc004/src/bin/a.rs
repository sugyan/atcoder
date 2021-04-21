use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    let mut max = 0;
    for i in 0..n - 1 {
        for j in i..n {
            let d = (xy[i].0 - xy[j].0).pow(2) + (xy[i].1 - xy[j].1).pow(2);
            max = max.max(d);
        }
    }
    println!("{:.6}", (max as f64).sqrt());
}
