use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut v = Vec::new();
    for i in 1..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            v.push(i);
            if i * i != n {
                v.push(n / i);
            }
        }
    }
    v.sort_unstable();
    for &v in &v {
        println!("{}", v);
    }
}
