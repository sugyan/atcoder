use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [f64; n],
    }
    println!("{:.15}", x.iter().map(|x| x.abs()).sum::<f64>());
    println!(
        "{:.15}",
        x.iter().map(|x| x.abs() * x.abs()).sum::<f64>().sqrt()
    );
    println!("{:.15}", x.iter().fold(0.0_f64, |acc, x| acc.max(x.abs())));
}
