use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                if (xy[i].0 - xy[k].0) * (xy[j].1 - xy[k].1)
                    == (xy[i].1 - xy[k].1) * (xy[j].0 - xy[k].0)
                {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
