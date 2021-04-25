use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i32, y: i32,
        n: usize,
        xy: [(i32, i32); n],
    }
    let mut answer = std::f64::MAX;
    for i in 0..n {
        let dx = xy[(i + 1) % n].0 - xy[i].0;
        let dy = xy[(i + 1) % n].1 - xy[i].1;
        let ex = x - xy[i].0;
        let ey = y - xy[i].1;
        answer = answer.min((dx * ey - dy * ex).abs() as f64 / ((dx * dx + dy * dy) as f64).sqrt());
    }
    println!("{:.10}", answer);
}
