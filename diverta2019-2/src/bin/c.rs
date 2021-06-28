use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    a.sort_unstable();
    let (mut min, mut max) = (a[0], a[n - 1]);
    let mut ops = Vec::with_capacity(n - 1);
    for &a in a.iter().skip(1).take(n - 2) {
        if a > 0 {
            ops.push((min, a));
            min -= a;
        } else {
            ops.push((max, a));
            max -= a;
        }
    }
    println!("{}", max - min);
    for (x, y) in &ops {
        println!("{} {}", x, y);
    }
    println!("{} {}", max, min);
}
