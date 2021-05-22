use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    let m = n * 100 / 108;
    for i in &[0, 1] {
        if (m + i) * 108 / 100 == n {
            println!("{}", m + i);
            return;
        }
    }
    println!(":(");
}
