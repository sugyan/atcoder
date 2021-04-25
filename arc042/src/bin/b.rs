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
        let dx = xy[i].0 - xy[(i + 1) % n].0;
        let dy = xy[i].1 - xy[(i + 1) % n].1;
        let d = match (dx == 0, dy == 0) {
            (true, _) => (xy[i].0 - x).abs() as f64,
            (_, true) => (xy[i].1 - y).abs() as f64,
            _ => {
                let a = dy as f64 / dx as f64;
                let b = xy[i].1 as f64 - a * xy[i].0 as f64;
                let c = y as f64 + x as f64 / a;
                let cx = a * (c - b) / (a * a + 1.0);
                let cy = (a * a * c + b) / (a * a + 1.0);
                ((cx - x as f64).powf(2.0) + (cy - y as f64).powf(2.0)).sqrt()
            }
        };
        answer = answer.min(d);
    }
    println!("{:.10}", answer);
}
