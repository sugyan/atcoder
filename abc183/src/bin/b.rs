use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: (f64, f64), g: (f64, f64),
    }
    let a = (s.1 + g.1) / (g.0 - s.0);
    println!("{:.10}", s.0 + s.1 / a)
}
