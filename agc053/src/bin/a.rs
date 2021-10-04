use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        _s: String,
        a: [i32; n + 1],
    }
    let k = (0..n).map(|i| (a[i] - a[i + 1]).abs()).min().unwrap();
    println!("{}", k);
    for j in 0..k {
        for &a in &a {
            print!("{} ", (a + j) / k);
        }
        println!();
    }
}
