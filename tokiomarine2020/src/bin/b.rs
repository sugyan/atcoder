use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64, v: i64,
        b: i64, w: i64,
        t: i64,
    }
    let yes = v > w && (a - b).abs() <= (v - w) * t;
    println!("{}", if yes { "YES" } else { "NO" });
}
