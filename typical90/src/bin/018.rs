use proconio::{fastout, input};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        t: f64,
        l: f64, x: f64, y: f64,
        q: usize,
        e: [f64; q],
    }
    for &e in &e {
        let dx = x;
        let dy = y + l / 2.0 * (2.0 * PI * e / t).sin();
        let dz = l / 2.0 - l / 2.0 * (2.0 * PI * e / t).cos();
        println!("{:.9}", (dz).atan2((dx * dx + dy * dy).sqrt()) * 180.0 / PI);
    }
}
