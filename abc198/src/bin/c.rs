use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: f64, x: f64, y: f64,
    }
    let d = (x * x + y * y).sqrt();
    println!("{}", if r > d { 2 } else { (d / r).ceil() as u32 });
}
