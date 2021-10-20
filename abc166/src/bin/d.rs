use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
    }
    let v = (0_i64..)
        .take_while(|&i| i.pow(5) - (i - 1).pow(5) < 10_i64.pow(9))
        .collect::<Vec<_>>();
    for &a in &v {
        let a5 = a.pow(5);
        for b in &v {
            let b5 = b.pow(5);
            if (x - a5).abs() == b5 {
                println!("{} {}", a, b * if x - a5 == b5 { -1 } else { 1 });
                return;
            }
        }
    }
}
