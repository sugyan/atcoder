use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut v = Vec::new();
    let (mut x0, mut y0) = (false, false);
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];
            let x = x1 - x2;
            let y = y1 - y2;
            if x == 0 {
                x0 = true;
                continue;
            }
            if y == 0 {
                y0 = true;
                continue;
            }
            if x > 0 {
                v.push(((y as f64).atan2(x as f64), (x, y)));
            }
        }
    }
    let mut answer = 0;
    if x0 {
        answer += 2;
    }
    if y0 {
        answer += 2;
    }
    v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for i in 0..v.len() {
        if i == 0 || (v[i].1).1 * (v[i - 1].1).0 != (v[i].1).0 * (v[i - 1].1).1 {
            answer += 2;
        }
    }
    println!("{}", answer);
}
