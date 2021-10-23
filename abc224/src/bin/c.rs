use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut answer = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            let d1 = (xy[i].0 - xy[j].0, xy[i].1 - xy[j].1);
            for k in j + 1..n {
                let d2 = (xy[k].0 - xy[j].0, xy[k].1 - xy[j].1);
                if d1.0 * d2.1 != d1.1 * d2.0 {
                    answer += 1;
                }
            }
        }
    }
    println!("{}", answer);
}
